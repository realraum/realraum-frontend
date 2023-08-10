use std::future::Future;

use gloo_net::http::{Method, Request, RequestBuilder, Response};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use web_sys::{RequestCache, RequestMode};

lazy_static! {
    static ref SOUND_RX: Regex = Regex::new(r#"href='([^']*)'.*?>([^<]*)"#).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub name: String,
    pub url: String,
}

// pub async fn old_get_sounds() -> String {
//     log::info!("get_sounds");
//     let url: reqwest::Url = "http://licht.realraum.at:8080".parse().unwrap();
//     log::info!("url: {:?}", url);

//     let client = Client::builder().build().unwrap();
//     let req = client
//         .request(reqwest::Method::GET, url)
//         .fetch_mode_no_cors()
//         .build()
//         .unwrap();

//     log::info!("req: {:?}", req);

//     let res = client.execute(req).await.unwrap();

//     log::info!("res: {:?}", res);

//     let txt = res.text().await.unwrap();

//     // let resp = reqwest::get(url).await.unwrap();
//     // let body = resp.text().await.unwrap();
//     txt
// }

pub async fn get_sounds_strings() -> String {
    // "TEST_TXT".to_string()
    TEST_TXT.to_string()
}

pub async fn _get_sounds_strings() -> String {
    log::info!("get_sounds_strings");
    let req = RequestBuilder::new("http://licht.realraum.at:8080")
        .method(Method::GET)
        .mode(RequestMode::NoCors)
        .build()
        .unwrap();
    log::info!("req: {:?}", req);

    let resp = req.send().await.unwrap();
    log::info!("resp: {:?}", resp);

    let text = resp.text().await.unwrap();
    log::info!("text: {:?}", text);
    text
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
    let mut sounds = parse_sounds(&txt);
    sort_sounds(&mut sounds);
    sounds
}

pub async fn play_sound(url: String) -> Result<(), String> {
    let url = format!("http://licht.realraum.at:8080{}", url);
    log::info!("play_sound: {}", url);
    let req = RequestBuilder::new(&url)
        .method(Method::GET)
        .mode(RequestMode::NoCors)
        .cache(RequestCache::NoCache)
        .build()
        .unwrap();

    log::info!("req: {:?}", req);

    let resp = req.send().await.unwrap();

    log::info!("resp: {:#?}", resp);

    // if resp.status() != 200 {
    //     log::info!("Status not OK");
    //     log::info!("Status: {} ({})", resp.status(), resp.status_text());
    //     log::info!("Headers: {:#?}", resp.headers());
    //     log::info!("Body: {:#?}", resp.body());

    //     return Err(resp.status_text());
    // }

    // log::info!("Status OK");

    Ok(())
}

fn sort_sounds(sounds: &mut Vec<Sound>) {
    // // Sort alphabetically by name
    // sounds.sort_by(|a, b| a.name.cmp(&b.name));

    let mut hl_sounds = Vec::new();

    // Move all sounds starting with "hl-sounds" to the end
    //  (they are the most annoying ones)
    //  First, remove the sounds from the list
    //  and store them in a separate list (hl_sounds)
    // Then, append the hl_sounds list to the end of the sounds list
    sounds.retain(|sound| {
        if sound.name.starts_with(HL_SOUNDS_STRING) {
            hl_sounds.push(sound.clone());
            false
        } else {
            true
        }
    });
    sounds.append(&mut hl_sounds);
}

const TEST_TXT: &str = include_str!("../data/licht.realraum.at.html");
pub const HL_SOUNDS_STRING: &str = "hl-sounds";

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
