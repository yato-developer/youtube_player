//LIBRARY_PATH="/opt/homebrew/lib" DYLD_LIBRARY_PATH="/opt/homebrew/lib" cargo run


use youtube_player::{player::Player, yt::YoutubeClient};
use std::thread;
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

    let mut input = String::new();
    let youtube_player = YoutubePlayer::default();

    println!("検索ワード:");
    io::stdin()
    .read_line(&mut input)
    .expect("入力エラー");

    let search_result = youtube_player.yt.search(&input).await?;

    for (i, track) in search_result.iter().enumerate() {
        println!("{}: {}", i, track.0.0);
    }

    println!("どの曲を再生しますか?");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("入力エラー");
    let index: usize = input.trim().parse().expect("数字を入力してください");

    let id = search_result[index].0.1.clone();

    let url = youtube_player.yt.fetch_song_url(&id).await?;
    youtube_player.player.play(&url).unwrap();

    thread::park();

    Ok(())
}