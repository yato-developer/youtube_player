use crate::{ArtistName, ChannelName, PlaylistId, PlaylistName, SongId, SongName, SongUrl};
use std::fmt::Error;
use std::path::PathBuf;
use std::io;

use rustypipe::{
    client::{RustyPipe, RustyPipeQuery},
    model::MusicItem,
    param::StreamFilter,
};


pub struct YoutubeClient{
    client: RustyPipeQuery,
}

impl Default for YoutubeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubeClient{
    pub fn new() -> Self{
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        path.push("Feather");
        let rp = RustyPipe::builder().storage_dir(path).build().unwrap();
        let client = rp.query();
        //YoutubeClient { client: client }の省略
        YoutubeClient { client }
    }


    pub async fn search_and_play(&self) -> Result<String, String> {
        let mut input = String::new();

        println!("Search word");
        io::stdin()
        .read_line(&mut input)
        .expect("");


        let search_result = self.search(&input).await?;
        
        input.clear();
        println!("Which song would you like to play? ");
        io::stdin()
            .read_line(&mut input)
            .expect("");
            
        let index: usize = input.trim().parse().expect("");
        let id = search_result[index].0.1.clone();
        
        let url = self.fetch_song_url(&id).await?;
        
        Ok(url)
    }
    
    pub async fn search(&self, query : &str) -> Result<Vec<((SongName, SongId),Vec<ArtistName>)>, String>{



        match self.client.music_search_main(query).await {
        Ok(results) => {
            
            let mut search_result = vec![];
            
            for item in results.items.items{
                if let MusicItem::Track(data) = item {
                    let song_id_pair = (data.name, data.id);
                    let artist_names: Vec<String> =
                    data.artists.into_iter().map(|id| id.name).collect();
                search_result.push((song_id_pair, artist_names));

                }
            }

            for (i, track) in search_result.iter().enumerate() {
                println!("{}: {}", i, track.0.0);
            }

            Ok(search_result)
        }  
        Err(_) => Err("Error in Search Result".to_string()),
        }

        
    }

    pub async fn fetch_song_url(&self, id: &SongId) -> Result<SongUrl, String> {
        match self.client.player(&id).await {
            Ok(player) => match player.select_audio_stream(&StreamFilter::default()) {
                Some(stream) => Ok(stream.url.clone()),
                None => Err("Audio Stream not Found".to_string()),
            },
            Err(_) => Err("Link cannot be Found".to_string()),
        }
    }
}

