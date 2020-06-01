use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Enclosure {
    pub url: String,
    #[serde(rename = "type")]
    pub etype: String
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub guid: String,
    pub pdalink: String,
    pub author: String,
    pub category: String,
    pub enclosure: Option<Enclosure>,
    #[serde(rename = "pubDate")]
    pub pub_date: String
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub image: Image,

    #[serde(rename = "item", default)]
    pub items: Vec<Item>
}

#[derive(Debug, Deserialize)]
pub struct Rss {
    pub version: String,
    pub channel: Channel
}

