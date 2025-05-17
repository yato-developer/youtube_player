//LIBRARY_PATH="/opt/homebrew/lib" DYLD_LIBRARY_PATH="/opt/homebrew/lib" cargo run


use youtube_player::{player::Player, yt::YoutubeClient};
use std::io;

struct YoutubePlayer{
    player : Player,
    yt : YoutubeClient
}

impl Default for YoutubePlayer{
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubePlayer{
    pub fn new() -> Self{
        let yt = YoutubeClient::default();
        let player = Player::new().unwrap();

        YoutubePlayer { player, yt }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let youtube_player = YoutubePlayer::default();

    let search_result_url = youtube_player.yt.search_and_play().await?;
    youtube_player.player.play(&search_result_url).unwrap();
    loop {
        let mut input = String::new();
        println!("Enter command (1:Play/2:Pause/3:Search):");
        io::stdin()
            .read_line(&mut input)
            .expect("");

        match input.trim() {
            "1" => {
                youtube_player.player.player_controller(youtube_player::player::PlayerControllerStatus::Playing);
            },
            "2" => {
                youtube_player.player.player_controller(youtube_player::player::PlayerControllerStatus::Paused);
            },
            "3" => {
                if let Ok(url) = youtube_player.yt.search_and_play().await {
                    youtube_player.player.play(&url).unwrap();
                }
            },
            _ => {
                println!("無効なコマンドです");
                continue;
            }
        };
    }
}