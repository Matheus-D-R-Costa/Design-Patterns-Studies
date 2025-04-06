#![allow(dead_code)]

trait VlcMediaPlayer {
    fn play_vlc(&self, file_name: &str);
}

trait Mp4MediaPlayer {
    fn play_mp4(&self, file_name: &str);
}

struct VlcPlayer;
impl VlcMediaPlayer for VlcPlayer {
    fn play_vlc(&self, file_name: &str) {
        println!("Tocando arquivo vlc: {}", file_name);
    }
}

struct Mp4Player;
impl Mp4MediaPlayer for Mp4Player {
    fn play_mp4(&self, file_name: &str) {
        println!("Tocando arquivo mp4: {}", file_name);
    }
}

pub enum MediaType {
    Vlc(String),
    Mp4(String),
}

pub trait AdvancedMediaPlayer {
    fn play(&self, media: MediaType);
}

pub struct AdvancedMediaPlayerImpl {
    vlc_player: VlcPlayer,
    mp4_player: Mp4Player,
}

impl AdvancedMediaPlayerImpl {
    pub fn new() -> Self {
        Self {
            vlc_player: VlcPlayer,
            mp4_player: Mp4Player,
        }
    }
}

impl AdvancedMediaPlayer for AdvancedMediaPlayerImpl {
    fn play(&self, media: MediaType) {
        match media {
            MediaType::Vlc(file_name) => self.vlc_player.play_vlc(&file_name),
            MediaType::Mp4(file_name) => self.mp4_player.play_mp4(&file_name),
        }
    }
}