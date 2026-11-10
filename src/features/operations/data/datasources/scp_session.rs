use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crate::core::constants::ssh::{
    SCP_COMMAND, SCP_LINE_BREAKS, SCP_POLL_MS, SCP_READ_CHUNK, SCP_RECURSIVE_PRESERVE,
    SSH_BATCH_MODE, SSH_OPTION_FLAG, SSH_PORT_FLAG,
};
use crate::core::helpers::files::tree_size::tree_size;
use crate::features::operations::data::datasources::pty_pair::pty_pair;
use crate::features::operations::data::datasources::scp_progress::scp_progress;
use crate::features::operations::domain::entities::scp_job::ScpJob;
use crate::features::operations::domain::entities::scp_progress::ScpProgress;
use crate::features::operations::domain::repositories::progress_sink::ProgressSink;

pub fn scp_session(job: &ScpJob, sink: &dyn ProgressSink) -> io::Result<()> {
    let (master, follower) = pty_pair()?;
    let mut command = Command::new(SCP_COMMAND);
    command.args([SCP_RECURSIVE_PRESERVE, SSH_OPTION_FLAG, SSH_BATCH_MODE]);
    if let Some(port) = &job.port {
        command.args([SSH_PORT_FLAG, port]);
    }
    command
        .arg(&job.source)
        .arg(&job.target)
        .stdin(Stdio::null())
        .stdout(Stdio::from(follower))
        .stderr(Stdio::null());
    unsafe {
        command.pre_exec(take_terminal);
    }
    let mut child = command.spawn()?;
    drop(command);
    let progress = read_progress(master);
    let mut meter = Meter::default();
    let finished = watch(&mut child, &progress, &mut meter, sink);
    meter.drain(&progress, sink);
    if finished? {
        meter.settle(tree_size(&job.local), sink);
        return Ok(());
    }
    Err(io::Error::from(ErrorKind::Other))
}

fn take_terminal() -> io::Result<()> {
    rustix::process::setsid()?;
    rustix::process::ioctl_tiocsctty(io::stdout())?;
    Ok(())
}

fn watch(
    child: &mut Child,
    progress: &Receiver<ScpProgress>,
    meter: &mut Meter,
    sink: &dyn ProgressSink,
) -> io::Result<bool> {
    loop {
        meter.drain(progress, sink);
        if sink.is_cancelled() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::from(ErrorKind::Interrupted));
        }
        if let Some(status) = child.try_wait()? {
            return Ok(status.success());
        }
        thread::sleep(Duration::from_millis(SCP_POLL_MS));
    }
}

fn read_progress(mut master: File) -> Receiver<ScpProgress> {
    let (sender, progress) = mpsc::channel();
    thread::spawn(move || {
        let mut buffer = vec![0_u8; SCP_READ_CHUNK];
        let mut pending = String::new();
        while let Ok(read) = master.read(&mut buffer) {
            if read == 0 {
                break;
            }
            pending.push_str(&String::from_utf8_lossy(&buffer[..read]));
            while let Some(end) = pending.find(SCP_LINE_BREAKS) {
                let line: String = pending.drain(..=end).collect();
                if let Some(found) = scp_progress(&line)
                    && sender.send(found).is_err()
                {
                    return;
                }
            }
        }
    });
    progress
}

#[derive(Default)]
struct Meter {
    name: String,
    bytes: u64,
    sent: u64,
}

impl Meter {
    fn drain(&mut self, progress: &Receiver<ScpProgress>, sink: &dyn ProgressSink) {
        while let Ok(found) = progress.try_recv() {
            if found.name != self.name {
                sink.set_item(&found.name);
                self.name = found.name;
                self.bytes = 0;
            }
            if found.bytes > self.bytes {
                let delta = found.bytes - self.bytes;
                sink.advance(delta);
                self.sent += delta;
                self.bytes = found.bytes;
            }
        }
    }

    fn settle(&mut self, total: u64, sink: &dyn ProgressSink) {
        if total > self.sent {
            sink.advance(total - self.sent);
            self.sent = total;
        }
    }
}
