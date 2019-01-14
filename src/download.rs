use reqwest;
use select::document::Document;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::error::Error;

pub fn download_page(link: &String) -> Option<Document> {
    reqwest::get(link)
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

fn get_download_link(client: &reqwest::Client, download_link: &String, video_id: &u32) -> Result<String, Box<Error>> {
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

fn create_file(folder_name: &String, file_name: &String) -> Result<File, Box<Error>> {
    let path = Path::new(folder_name).join(file_name);
    Ok(File::create(path)?)
}

pub fn download_video(client: &reqwest::Client, download_link: &String, video_id: &u32, folder_name: &String) -> Result<String, Box<Error>> {
    let link = get_download_link(client, download_link, video_id)?;
    let mut response = reqwest::get(&link)?;
    let file_name = format!("{}.mp4", video_id);
    let mut file = create_file(folder_name, &file_name)?;
    copy(&mut response, &mut file)?;
    return Ok(file_name);
}