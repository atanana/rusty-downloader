use select::document::Document;
use select::predicate::{Class, Name, Predicate};

pub fn parse_video_ids(document: &Document) -> Vec<u32> {
    let link_selector = Class("list-video-result")
        .descendant(Name("a"));
    return document.find(link_selector)
        .flat_map(|link| link.attr("data-id").and_then(|id| id.parse::<u32>().ok()))
        .collect();
}

pub fn parse_pages_count(document: &Document) -> Option<u32> {
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