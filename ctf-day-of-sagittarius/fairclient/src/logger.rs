use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::interface::update_hud;

pub struct BufferLogger {
    messages: Arc<Mutex<VecDeque<String>>>,
    pub max_size: usize,
}

impl BufferLogger {
    pub fn new(max_size: usize) -> BufferLogger {
        BufferLogger {
            messages: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            max_size,
        }
    }

    pub fn log(&self, text: &str) {
        {
            let mut messages = self.messages.lock().unwrap();
            if messages.len() >= self.max_size {
                messages.pop_front();
            }
            messages.push_back(text.to_string());
        }

        update_hud();
    }

    pub fn get_messages(&self) -> VecDeque<String> {
        let messages = self.messages.lock().unwrap();
        messages.clone()
    }
}