use std::collections::HashMap;

use tokio::{sync::oneshot, task::JoinHandle};

pub struct Task {
    terminator: oneshot::Sender<()>,
    handle: JoinHandle<()>,
}

impl Task {
    pub fn new(terminator: oneshot::Sender<()>, handle: JoinHandle<()>) -> Self {
        Self { terminator, handle }
    }

    pub fn kill(self) {
        match self.terminator.send(()) {
            Ok(_) => {
                if !self.handle.is_finished() {
                    self.handle.abort();
                }
            }
            Err(_) => {
                // log
            }
        }
    }
}
