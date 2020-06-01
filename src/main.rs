mod rss;
use bytes::buf::BufExt as _;
use hyper_tls::HttpsConnector;
use hyper::{Client, Uri};
use rss::Rss;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()>{
    let url: Uri = "https://www.vedomosti.ru/rss/news".parse()?;
    let rss_body = fetch_xml(url).await?;
    let items = rss_body.channel.items.iter().filter(|item| item.category == "Технологии").take(10);
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
    let rss: Rss = serde_xml_rs::from_reader(body.reader())?;
    Ok(rss)
}
