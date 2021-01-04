#[macro_use]
extern crate serde_derive;

mod parse;
mod download;

use reqwest::Client;
use select::document::Document;
use std::env;
use std::fs;
use std::path::Path;
use std::error::Error;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    match parse_args() {
        Ok((page_link, download_link, folder_name)) => {
            download_videos(&page_link, &download_link, &folder_name);
        }
        _ => panic!("Incorrect arguments!")
    }
}

fn parse_args() -> Result<(String, String, String), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let page_link: String = args[1].parse()?;
    let download_link: String = args[2].parse()?;
    let folder_name: String = args[3].parse()?;
    return Ok((page_link, download_link, folder_name));
}

fn download_videos(page_link: &String, download_link: &String, folder_name: &String) {
    let client = Client::new();
    fs::create_dir_all(Path::new(folder_name))
        .expect("Cannot create folder for downloads");
    let video_ids = get_video_ids(&client, page_link);
    let videos_count = video_ids.len();
    println!("Downloading {} videos", videos_count);

    let counter = Arc::new(Mutex::new(0u32));
    video_ids.par_iter()
        .for_each_with(counter, |counter, video_id| {
            let current = get_current(counter);
            println!("Start download video {} ({} of {})", video_id, current, videos_count);
            match download::download_video(&client, download_link, &video_id, folder_name) {
                Ok(file_name) => println!("Downloaded to {}  ({} of {})", file_name, current, videos_count),
                _ => println!("Cannot download {}", video_id)
            }
        })
}

fn get_video_ids(client: &Client, link: &String) -> Vec<u32> {
    let pages_count = get_pages_count(&client, link);
    download_all_pages(client, link, pages_count)
        .iter()
        .flat_map(|doc| parse::parse_video_ids(&doc))
        .collect()
}

fn get_pages_count(client: &Client, page_link: &String) -> u32 {
    download::download_page(&client, page_link)
        .and_then(|page| parse::parse_pages_count(&page))
        .unwrap_or(1)
}

fn download_all_pages(client: &Client, link: &String, pages_count: u32) -> Vec<Document> {
    (1..=pages_count)
        .map(|page| format!("{}?page={}", link, page))
        .map(|page_link| download::download_page(client, &page_link))
        .filter_map(|r| r)
        .collect()
}

fn get_current(counter: &Arc<Mutex<u32>>) -> u32 {
    let mut current = counter.lock().unwrap();
    *current += 1;
    *current
}