#![allow(dead_code)]

use std::{collections::HashMap, path::Path};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(dir: &Path, filename: &str);

#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Subscriber) {
        if let Some(listeners) = self.events.get_mut(&event_type) {
            listeners.retain(|&x| !std::ptr::fn_addr_eq(x, listener));
        }
    }

    pub fn notify(&self, event_type: Event, dir: &Path, filename: &str) {
        if let Some(listeners) = self.events.get(&event_type) {
            for listener in listeners {
                listener(dir, filename);
            }
        }
    }
}
