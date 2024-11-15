#![allow(dead_code)]
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct UpdateGui {
    update_gui: Arc<Mutex<bool>>
}

impl Default for UpdateGui {
    fn default() -> Self {
        Self {
            update_gui: Arc::new(Mutex::new(true))
        }
    }
}
impl UpdateGui {
    pub fn should_update(&self) -> bool {
        *self.update_gui.lock().unwrap()
    }

    pub fn set(&self, val: bool) {
        *self.update_gui.lock().unwrap() = val;
    }
}



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