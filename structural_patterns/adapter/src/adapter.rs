#![allow(dead_code)]

use crate::advanced_media_player::{AdvancedMediaPlayer, AdvancedMediaPlayerImpl, MediaType};

pub struct MediaAdapter {
    advanced_player: Box<dyn AdvancedMediaPlayer>,
}

impl MediaAdapter {
    pub fn new() -> Self {
        let player = Box::new(AdvancedMediaPlayerImpl::new());
        
        Self { advanced_player: player }
    }
    
    pub fn play(&self, audio_type: &str, file_name: &str) {
        let media = match audio_type {
            "vlc" => MediaType::Vlc(file_name.to_string()),
            "mp4" => MediaType::Mp4(file_name.to_string()),
            _ => return, // Não suportado pelo adaptador
        };
        
        self.advanced_player.play(media);
    }
}