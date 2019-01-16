use reqwest::Client;
use select::document::Document;
use std::fs::{File, rename};
use std::io::copy;
use std::path::Path;
use std::error::Error;

pub fn download_page(client: &Client, link: &String) -> Option<Document> {
    client.get(link)
        .send()
        .ok()
        .and_then(parse_page)
}

fn parse_page(response: reqwest::Response) -> Option<Document> {
    Document::from_read(response).ok()
}

#[derive(Deserialize, Debug)]
struct DownloadResponse {
    url: String,
    zona: bool,
}

fn get_download_link(client: &Client, download_link: &String, video_id: &u32) -> Result<String, Box<Error>> {
    let params = [
        ("id", video_id.to_string()),
        ("type", String::from("mp4"))
    ];
    let mut response = client.post(download_link)
        .form(&params)
        .send()?;
    let json: DownloadResponse = response.json()?;
    return Ok(json.url);
}

pub fn download_video(client: &Client, download_link: &String, video_id: &u32, folder_name: &String) -> Result<String, Box<Error>> {
    let link = get_download_link(client, download_link, video_id)?;
    let mut response = client.get(&link).send()?;
    let tmp_file_name = format!("{}.mp4.tmp", video_id);
    let tmp_path = Path::new(folder_name).join(tmp_file_name);
    let mut tmp_file = File::create(&tmp_path)?;
    copy(&mut response, &mut tmp_file)?;
    let file_name = format!("{}.mp4", video_id);
    let file_path = Path::new(folder_name).join(&file_name);
    rename(tmp_path, file_path)?;
    return Ok(file_name);
}