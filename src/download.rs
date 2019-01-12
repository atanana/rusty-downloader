use reqwest;
use select::document::Document;
use std::fs;
use std::io;
use std::path::Path;

pub fn download_page(link: &String) -> Option<Document> {
    reqwest::get(link)
        .ok()
        .and_then(|page| Document::from_read(page).ok())
}

pub fn download_video(link: &String, video_id: u32, folder_name: &String) -> Result<u64, io::Error> {
    let mut response = match reqwest::get(link) {
        Ok(response) => response,
        Err(e) => return Err(io::Error::new(io::ErrorKind::ConnectionRefused, e))
    };
    let mut dest = {
        let path = Path::new(folder_name).join(format!("{}.mp4", video_id));
        fs::File::create(path)?
    };
    return io::copy(&mut response, &mut dest);
}

#[derive(Deserialize, Debug)]
struct DownloadResponse {
    url: String,
    zona: bool,
}

pub fn get_download_link(client: &reqwest::Client, download_link: &String, video_id: u32) -> Option<String> {
    let params = [
        ("id", video_id.to_string()),
        ("type", "mp4".to_owned())
    ];
    return client.post(download_link)
        .form(&params)
        .send()
        .ok()
        .and_then(|mut response| response.json::<DownloadResponse>().ok())
        .map(|json| json.url);
}