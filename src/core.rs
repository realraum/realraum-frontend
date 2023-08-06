use gloo_net::http::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, RequestBuilder};

lazy_static! {
    static ref SOUND_RX: Regex = Regex::new(r#"href='([^']*)'.*?>([^<]*)"#).unwrap();
}

pub async fn old_get_sounds() -> String {
    log::info!("get_sounds");
    let url: reqwest::Url = "http://licht.realraum.at:8080".parse().unwrap();
    log::info!("url: {:?}", url);

    let client = Client::builder().build().unwrap();
    let req = client
        .request(reqwest::Method::GET, url)
        .fetch_mode_no_cors()
        .build()
        .unwrap();

    log::info!("req: {:?}", req);

    let res = client.execute(req).await.unwrap();

    log::info!("res: {:?}", res);

    let txt = res.text().await.unwrap();

    // let resp = reqwest::get(url).await.unwrap();
    // let body = resp.text().await.unwrap();
    txt
}

pub async fn get_sounds() -> String {
    let resp = Request::get("http://licht.realraum.at:8080")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    String::new()
}
