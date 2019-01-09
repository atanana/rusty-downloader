use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let link: String = args[1].parse().unwrap();
    let folder: String = args[2].parse().unwrap();
    let video_ids = download_page(&link)
        .and_then(|page| parse_pages_count(&page))
        .map(|pages_count| download_all_pages(&link, pages_count))
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

fn download_all_pages(link: &String, pages_count: u32) -> Vec<u32> {
    (1..=pages_count)
        .map(|page| format!("{}&page={}", link, page))
        .flat_map(|page_link| {
            download_page(&page_link).map_or(Vec::new(), |doc| parse_video_ids(&doc))
        })
        .collect()
}

fn parse_video_ids(document: &Document) -> Vec<u32> {
    vec![0]
}
