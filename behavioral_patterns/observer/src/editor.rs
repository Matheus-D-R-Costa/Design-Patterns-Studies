#![allow(dead_code)]

use std::path::Path;

use crate::observer::{Event, Publisher};

pub struct Editor<'a> {
    pub publisher: Publisher,
    pub file_path: (&'a Path, &'a str)
}

impl<'a> Editor<'a> {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn load(&mut self, file_path: (&'a Path, &'a str)) {
        self.file_path = file_path;
        self.publisher.notify(Event::Load, self.file_path.0, self.file_path.1);
    }

    pub fn save(&self) {
        self.publisher.notify(Event::Save, self.file_path.0, self.file_path.1);
    }
}
