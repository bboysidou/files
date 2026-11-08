use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::core::constants::network::{
    GIO_COMMAND, GIO_LOCALE, GIO_MOUNT, MOUNT_ALREADY, MOUNT_TIMEOUT_SECS, PROMPT_READ_CHUNK,
};
use crate::features::network::data::datasources::gio_prompt_answer::gio_prompt_answer;
use crate::features::network::domain::entities::network_credentials::NetworkCredentials;
use crate::features::network::domain::entities::network_mount_error::NetworkMountError;

pub fn gio_mount_session(
    uri: &str,
    credentials: Option<&NetworkCredentials>,
) -> Result<(), NetworkMountError> {
    let failed = |error: std::io::Error| NetworkMountError::Failed(error.to_string());
    let mut child = Command::new(GIO_COMMAND)
        .args([GIO_MOUNT, uri])
        .env("LC_ALL", GIO_LOCALE)
        .env("LANGUAGE", GIO_LOCALE)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(failed)?;
    let pipes = (child.stdout.take(), child.stdin.take());
    let child = Arc::new(Mutex::new(child));
    let timed_out = Arc::new(AtomicBool::new(false));
    let (done, watchdog) = watch(child.clone(), timed_out.clone());
    let answered = match pipes {
        (Some(stdout), Some(stdin)) => answer_prompts(stdout, stdin, credentials),
        _ => Ok(()),
    };
    let _ = done.send(());
    let _ = watchdog.join();
    let child = Arc::try_unwrap(child)
        .map_err(|_| NetworkMountError::TimedOut)?
        .into_inner()
        .map_err(|_| NetworkMountError::TimedOut)?;
    finish(child, answered, timed_out.load(Ordering::SeqCst))
}

fn watch(
    child: Arc<Mutex<Child>>,
    timed_out: Arc<AtomicBool>,
) -> (mpsc::Sender<()>, thread::JoinHandle<()>) {
    let (done, finished) = mpsc::channel::<()>();
    let watchdog = thread::spawn(move || {
        let limit = Duration::from_secs(MOUNT_TIMEOUT_SECS);
        if finished.recv_timeout(limit) == Err(RecvTimeoutError::Timeout) {
            timed_out.store(true, Ordering::SeqCst);
            if let Ok(mut child) = child.lock() {
                let _ = child.kill();
            }
        }
    });
    (done, watchdog)
}

fn finish(
    mut child: Child,
    answered: Result<(), NetworkMountError>,
    timed_out: bool,
) -> Result<(), NetworkMountError> {
    if let Err(error) = answered {
        let _ = child.kill();
        let _ = child.wait();
        return Err(error);
    }
    let output = child
        .wait_with_output()
        .map_err(|error| NetworkMountError::Failed(error.to_string()))?;
    if timed_out {
        return Err(NetworkMountError::TimedOut);
    }
    let complaint = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if output.status.success() || complaint.contains(MOUNT_ALREADY) {
        return Ok(());
    }
    Err(NetworkMountError::Failed(complaint))
}

fn answer_prompts(
    mut stdout: ChildStdout,
    mut stdin: ChildStdin,
    credentials: Option<&NetworkCredentials>,
) -> Result<(), NetworkMountError> {
    let failed = |error: std::io::Error| NetworkMountError::Failed(error.to_string());
    let (mut chunk, mut line) = ([0_u8; PROMPT_READ_CHUNK], Vec::new());
    let mut password_sent = false;
    loop {
        let read = stdout.read(&mut chunk).map_err(failed)?;
        if read == 0 {
            return Ok(());
        }
        for byte in &chunk[..read] {
            if *byte == b'\n' {
                line.clear();
            } else {
                line.push(*byte);
            }
        }
        let text = String::from_utf8_lossy(&line).into_owned();
        if let Some(answer) = gio_prompt_answer(&text, credentials, &mut password_sent)? {
            reply(&mut stdin, &answer).map_err(failed)?;
            line.clear();
        }
    }
}

fn reply(stdin: &mut ChildStdin, answer: &str) -> std::io::Result<()> {
    stdin.write_all(answer.as_bytes())?;
    stdin.write_all(b"\n")?;
    stdin.flush()
}
