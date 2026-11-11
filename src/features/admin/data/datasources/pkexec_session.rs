use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard};

use crate::core::constants::admin::{
    ADMIN_HELPER_FLAG, PKEXEC_COMMAND, PKEXEC_DISMISSED_CODE, PKEXEC_NO_INTERNAL_AGENT,
};
use crate::features::admin::data::models::admin_reply::AdminReply;
use crate::features::admin::data::models::admin_request::AdminRequest;
use crate::features::admin::domain::entities::admin_error::AdminError;
use crate::features::admin::domain::repositories::admin_session::AdminSession;

struct Channel {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
}

impl Channel {
    fn receive(&mut self) -> Option<AdminReply> {
        let mut line = String::new();
        match self.output.read_line(&mut line) {
            Ok(0) | Err(_) => None,
            Ok(_) => serde_json::from_str(&line).ok(),
        }
    }

    fn exchange(&mut self, request: &AdminRequest) -> Option<AdminReply> {
        let mut line = serde_json::to_vec(request).ok()?;
        line.push(b'\n');
        self.input.write_all(&line).ok()?;
        self.input.flush().ok()?;
        self.receive()
    }

    fn close_slot(slot: &mut Option<Channel>) {
        if let Some(mut channel) = slot.take() {
            drop(channel.input);
            let _ = channel.child.wait();
        }
    }
}

#[derive(Default)]
pub struct PkexecSession {
    channel: Mutex<Option<Channel>>,
    active: AtomicBool,
}

impl PkexecSession {
    fn lock(&self) -> MutexGuard<'_, Option<Channel>> {
        self.channel
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn spawn() -> Result<Channel, AdminError> {
        let program = env::current_exe().map_err(|_| AdminError::Failed)?;
        let mut child = Command::new(PKEXEC_COMMAND)
            .arg(PKEXEC_NO_INTERNAL_AGENT)
            .arg(program)
            .arg(ADMIN_HELPER_FLAG)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| AdminError::Failed)?;
        let (Some(input), Some(output)) = (child.stdin.take(), child.stdout.take()) else {
            let _ = child.kill();
            return Err(AdminError::Failed);
        };
        let mut channel = Channel {
            child,
            input,
            output: BufReader::new(output),
        };
        if matches!(channel.receive(), Some(AdminReply::Ready)) {
            return Ok(channel);
        }
        drop(channel.input);
        let status = channel.child.wait().map_err(|_| AdminError::Failed)?;
        Err(match status.code() {
            Some(PKEXEC_DISMISSED_CODE) => AdminError::Cancelled,
            _ => AdminError::Failed,
        })
    }

    pub fn call(&self, request: &AdminRequest) -> Option<AdminReply> {
        let mut slot = self.lock();
        let reply = slot.as_mut()?.exchange(request);
        if reply.is_none() {
            self.active.store(false, Ordering::Release);
            Channel::close_slot(&mut slot);
        }
        reply
    }
}

impl AdminSession for PkexecSession {
    fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    fn start(&self) -> Result<(), AdminError> {
        let mut slot = self.lock();
        let alive = slot
            .as_mut()
            .is_some_and(|channel| matches!(channel.child.try_wait(), Ok(None)));
        if !alive {
            Channel::close_slot(&mut slot);
            *slot = Some(Self::spawn()?);
        }
        self.active.store(true, Ordering::Release);
        Ok(())
    }

    fn end(&self) {
        self.active.store(false, Ordering::Release);
        Channel::close_slot(&mut self.lock());
    }
}
