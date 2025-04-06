pub mod media_player;
pub mod advanced_media_player;
pub mod adapter;

#[cfg(test)]
mod tests {
    use crate::media_player::{AudioPlayer, MediaPlayer};

    #[test]
    fn test_mp3_playback() {
        let mut player = AudioPlayer::new();
        player.play("mp3", "música.mp3");
    }

    #[test]
    fn test_vlc_playback() {
        let mut player = AudioPlayer::new();
        player.play("vlc", "filme.vlc");
    }

    #[test]
    fn test_mp4_playback() {
        let mut player = AudioPlayer::new();
        player.play("mp4", "vídeo.mp4");
    }

    #[test]
    fn test_unsupported_format() {
        let mut player = AudioPlayer::new();
        player.play("avi", "vídeo.avi");
    }

    #[test]
    fn test_adapter_lazy_loading() {
        let mut player = AudioPlayer::new();
        

        player.play("vlc", "filme1.vlc");
        

        player.play("mp4", "vídeo1.mp4");
    }
}