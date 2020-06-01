use bytes::buf::BufExt as _;
use hyper_tls::HttpsConnector;
use hyper::{Client, Uri};
use serde_derive::Deserialize;
//#[macro_use]
//extern crate serde_derive;

//use serde_xml_rs::{from_reader};

#[derive(Debug, Deserialize)]
struct Enclosure {
    url: String,
    #[serde(rename = "type")]
    etype: String
}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    link: String,
    guid: String,
    pdalink: String,
    author: String,
    category: String,
    enclosure: Option<Enclosure>,
    #[serde(rename = "pubDate")]
    pub_date: String
}

#[derive(Debug, Deserialize)]
struct Image {
    url: String,
    title: String,
    link: String
}

#[derive(Debug, Deserialize)]
struct Channel {
    title: String,
    link: String,
    description: String,
    image: Image,

    #[serde(rename = "item", default)]
    items: Vec<Item>
}

#[derive(Debug, Deserialize)]
struct Rss {
    version: String,
    channel: Channel
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()>{
    let url: Uri = "https://www.vedomosti.ru/rss/news".parse()?;
    let rss = fetch_xml(url).await?;
    let items = rss.channel.items.iter().filter(|item| item.category == "Технологии").take(10);
    for item in items {
        println!("{} | {}", item.title, item.link);
    }

    Ok(())
}

async fn fetch_xml(url: Uri) -> Result<Rss> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let rss = serde_xml_rs::from_reader(body.reader())?;
    Ok(rss)
}
