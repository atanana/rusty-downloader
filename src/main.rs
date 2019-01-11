#[macro_use]
extern crate serde_derive;

use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_link: String = args[1].parse().unwrap();
    let download_link: String = args[2].parse().unwrap();
    let folder_name: String = args[3].parse().unwrap();
    test(&download_link, &folder_name);
}

fn test(download_link: &String, folder_name: &String) {
    let video_id = 788911;
    let client = reqwest::Client::new();
    let link = get_download_link(&client, download_link, video_id).unwrap();
    download_video(&link, video_id, folder_name);
}

fn download_video(link: &String, video_id: u32, folder_name: &String) -> Result<u64, io::Error> {
    let mut response = reqwest::get(link).unwrap();
    let mut dest = {
        let path = Path::new(folder_name).join(format!("{}.mp4", video_id));
        fs::File::create(path)?
    };
    return io::copy(&mut response, &mut dest);
}

fn download_videos(page_link: &String, download_link: &String, folder: &String) {
    let video_ids = download_page(&page_link)
        .and_then(|page| parse_pages_count(&page))
        .map(|pages_count| get_video_ids(&page_link, pages_count))
        .unwrap();
    println!("{:?}", video_ids);
}

fn download_page(link: &String) -> Option<Document> {
    reqwest::get(link)
        .ok()
        .and_then(|page| Document::from_read(page).ok())
}

fn parse_pages_count(document: &Document) -> Option<u32> {
    let link_selector = Class("paginator")
        .descendant(Name("ul"))
        .descendant(Name("li"))
        .descendant(Name("a"));
    return document
        .find(link_selector)
        .map(|link| link.text())
        .map(|link| link.parse::<u32>())
        .filter_map(Result::ok)
        .max();
}

fn get_video_ids(link: &String, pages_count: u32) -> Vec<u32> {
    download_all_pages(link, pages_count)
        .iter()
        .flat_map(|doc| parse_video_ids(&doc))
        .collect()
}

fn download_all_pages(link: &String, pages_count: u32) -> Vec<Document> {
    (1..=pages_count)
        .map(|page| format!("{}&page={}", link, page))
        .map(|page_link| download_page(&page_link))
        .filter_map(|r| r)
        .collect()
}

fn parse_video_ids(document: &Document) -> Vec<u32> {
    let link_selector = Class("list-video-result")
        .descendant(Name("a"));
    return document.find(link_selector)
        .flat_map(|link| link.attr("data-id").and_then(|id| id.parse::<u32>().ok()))
        .collect();
}

#[derive(Deserialize, Debug)]
struct DownloadResponse {
    url: String,
    zona: bool,
}

fn get_download_link(client: &reqwest::Client, download_link: &String, video_id: u32) -> Option<String> {
    let params = [
        ("id", "788911"),
        ("type", "mp4")
    ];
    return client.post(download_link)
        .form(&params)
        .send()
        .ok()
        .and_then(|mut response| response.json::<DownloadResponse>().ok())
        .map(|json| json.url);
}