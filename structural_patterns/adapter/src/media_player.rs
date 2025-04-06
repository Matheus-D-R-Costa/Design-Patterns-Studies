#![allow(dead_code)]

use crate::adapter::MediaAdapter;

pub trait MediaPlayer {
    fn play(&mut self, audio_type: &str, file_name: &str);
}

pub struct AudioPlayer {
    adapter: Option<MediaAdapter>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self { adapter: None }
    }
    
    fn get_adapter(&mut self) -> &MediaAdapter {
        if self.adapter.is_none() {
            self.adapter = Some(MediaAdapter::new());
        }
        self.adapter.as_ref().unwrap()
    }
}

impl MediaPlayer for AudioPlayer {
    fn play(&mut self, audio_type: &str, file_name: &str) {
        match audio_type {
            "mp3" => println!("Tocando arquivo mp3: {}", file_name),
            "vlc" | "mp4" => {
                let adapter = self.get_adapter();
                adapter.play(audio_type, file_name);
            },
            _ => println!("Formato {} não suportado", audio_type),
        }
    }
}