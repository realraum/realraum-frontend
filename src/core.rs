use std::future::Future;

use gloo_net::http::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref SOUND_RX: Regex = Regex::new(r#"href='([^']*)'.*?>([^<]*)"#).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub name: String,
    pub url: String,
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

pub async fn get_sounds_strings() -> String {
    // "TEST_TXT".to_string()
    TEST_TXT.to_string()
}

pub async fn _get_sounds_strings() -> String {
    let resp = Request::get("http://licht.realraum.at:8080")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    String::new()
}

pub fn parse_sounds(txt: &str) -> Vec<Sound> {
    let mut sounds = Vec::new();
    for cap in SOUND_RX.captures_iter(txt) {
        let name = cap[2].to_string();
        let url = cap[1].to_string();
        sounds.push(Sound { name, url });
    }
    sounds
}

pub async fn get_sounds() -> Vec<Sound> {
    let txt = get_sounds_strings().await;
    parse_sounds(&txt)
}

pub async fn play_sound(url: String) -> Result<(), String> {
    let url = format!("http://licht.realraum.at:8080{}", url);
    log::info!("play_sound: {}", url);
    let resp = Request::get(&url).send().await.unwrap();

    // let body = reqwest_wasm::blocking::get("https://www.rust-lang.org")?.text()?;
    if resp.status() != 200 {
        return Err(resp.status_text());
    }
    Ok(())
}

const TEST_TXT: &str = include_str!("../data/licht.realraum.at.html");

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_parse_sounds() {
//         let sounds = parse_sounds(TEST_TXT);
//         // assert_eq!(sounds.len(), 4);
//         assert_eq!(sounds[0].name, "sad_trombone.ogg");
//         assert_eq!(sounds[0].url, "/sound/sad_trombone.ogg");
//     }
// }
