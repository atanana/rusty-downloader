use std::env;
use reqwest;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let args: Vec<String> = env::args().collect();
    let link: String = args[1].parse().unwrap();
    let folder: String = args[2].parse().unwrap();
    let pages_count = download_page(&link)
        .and_then(|page| parse_pages_count(&page)).unwrap();
    println!("{}", pages_count)
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
    return document.find(link_selector)
        .map(|link| link.text())
        .map(|link| link.parse::<u32>())
        .filter_map(Result::ok)
        .max();
}