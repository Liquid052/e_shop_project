use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct AppExit {
    pub exit: Arc<Mutex<bool>>
}

impl AppExit {
    pub fn should_exit(&self) -> bool {
        *self.exit.lock().unwrap()
    }

    pub fn set(&self, val: bool) {
        *self.exit.lock().unwrap() = val;
    }
}