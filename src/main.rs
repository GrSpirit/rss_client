use bytes::buf::BufExt as _;
use hyper_tls::HttpsConnector;
use hyper::{Client, Uri};
#[macro_use]
extern crate serde_derive;

use serde_xml_rs::{from_reader};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Enclosure {
    url: String,
    #[serde(rename = "type")]
    etype: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    title: String,
    link: String,
    guid: String,
    pdalink: String,
    author: String,
    category: String,
    enclosure: Enclosure,
    #[serde(rename = "pubDate")]
    pub_date: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Image {
    url: String,
    title: String,
    link: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Channel {
    title: String,
    link: String,
    description: String,
    image: Image,

    #[serde(rename = "Item", default)]
    items: Vec<Item>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Rss {
    version: String,
    channel: Channel
}
/*  
 <item>
  <title>Один из крупнейших сервисов аренды автомобилей Hertz объявил о банкротстве</title>
  <link>https://www.vedomosti.ru/auto/news/2020/05/23/830918-hertz</link>
  <guid>https://www.vedomosti.ru/auto/news/2020/05/23/830918-hertz</guid>
  <pdalink>https://www.vedomosti.ru/auto/news/2020/05/23/830918-hertz</pdalink>
  <author></author>
  <category>Авто</category>
  <enclosure url="https://cdn.vdmsti.ru/image/2020/40/s3k3o/normal-10ez.jpg" type="image/jpeg"/>
  <pubDate>Sat, 23 May 2020 13:06:44 +0300</pubDate>
 </item>
*/

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()>{
    let test_xml = r##"<item>
        <title>Песков описал свое состояние при заболевании коронавирусом</title>
        <link>https://www.vedomosti.ru/society/news/2020/05/31/831529-sostoyanie</link>
        <guid>https://www.vedomosti.ru/society/news/2020/05/31/831529-sostoyanie</guid>
        <pdalink>https://www.vedomosti.ru/society/news/2020/05/31/831529-sostoyanie</pdalink>
        <author/>
        <category>Общество</category>
        <enclosure url="https://cdn.vdmsti.ru/image/2020/48/qahod/normal-y2o.jpg" type="image/jpeg"/>
        <pubDate>Sun, 31 May 2020 12:16:11 +0300</pubDate>
        </item>"##;
    let test_item: Item = serde_xml_rs::from_str(test_xml)?;
    println!("{:?}", test_item);
    let url: Uri = "https://www.vedomosti.ru/rss/news".parse()?;
    let rss = fetch_xml(url).await?;
    println!("{:?}", rss);
    for item in rss.channel.items {
        println!("{}", item.title);
    }

    Ok(())
}

async fn fetch_xml(url: Uri) -> Result<Rss> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;
    let rss = from_reader(body.reader())?;
    Ok(rss)
}
