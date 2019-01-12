#[macro_use]
extern crate serde_derive;

mod parse;
mod download;

use reqwest;
use select::document::Document;
use std::env;
use std::error::Error;

fn main() {
    match parse_args() {
        Ok((page_link, download_link, folder_name)) => {
            test(&download_link, &folder_name);
        }
        _ => eprintln!("Incorrect arguments!")
    }
}

fn parse_args() -> Result<(String, String, String), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let page_link: String = args[1].parse()?;
    let download_link: String = args[2].parse()?;
    let folder_name: String = args[3].parse()?;
    return Ok((page_link, download_link, folder_name));
}

fn test(download_link: &String, folder_name: &String) {
    let video_id = 788911;
    let client = reqwest::Client::new();
    let link = download::get_download_link(&client, download_link, &video_id).unwrap();
    download::download_video(&link, &video_id, folder_name);
}

fn download_videos(page_link: &String, download_link: &String, folder: &String) {
    let video_ids = download::download_page(&page_link)
        .and_then(|page| parse::parse_pages_count(&page))
        .map(|pages_count| get_video_ids(&page_link, pages_count))
        .unwrap();
    println!("{:?}", video_ids);
}

fn get_video_ids(link: &String, pages_count: u32) -> Vec<u32> {
    download_all_pages(link, pages_count)
        .iter()
        .flat_map(|doc| parse::parse_video_ids(&doc))
        .collect()
}

fn download_all_pages(link: &String, pages_count: u32) -> Vec<Document> {
    (1..=pages_count)
        .map(|page| format!("{}&page={}", link, page))
        .map(|page_link| download::download_page(&page_link))
        .filter_map(|r| r)
        .collect()
}