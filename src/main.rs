use std::env;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();
    let link: String = args[1].parse().unwrap();
    let folder: String = args[2].parse().unwrap();
    let page = download_page(&link).unwrap();
    println!("{}", page);
}

fn download_page(link: &String) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(link)?.text()?)
}