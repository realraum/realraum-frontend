use gloo_net::http::{Method, RequestBuilder};
use lazy_static::lazy_static;
use leptos::window;
// use regex::Regex;
use serde::{Deserialize, Serialize};
use web_sys::{RequestCache, RequestMode};

lazy_static! {
    // // static ref SOUND_RX: Regex = Regex::new(r#"href='([^']*)'.*?>([^<]*)"#).unwrap();
    static ref BASE_URL: String = window().origin();
    static ref SOUNDS_API_PATH: String = format!("{}/api/v1/sounds", BASE_URL.as_str());
    static ref KILLALL_URL: String = format!("{}/api/v1/killall_mplayer", BASE_URL.as_str());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub name: String,
    pub url: String,
    pub play_count: i64,
}

pub async fn _get_sounds_strings() -> String {
    "TEST_TXT".to_string()
    // TEST_TXT.to_string()
}

pub async fn get_sounds_strings() -> Result<String, gloo_net::Error> {
    let req = RequestBuilder::new(&SOUNDS_API_PATH)
        .method(Method::GET)
        .mode(RequestMode::Cors)
        .build()?;

    let resp = req.send().await?;

    resp.text().await
}

// pub fn _parse_sounds(txt: &str) -> Vec<Sound> {
//     let mut sounds = Vec::new();
//     for cap in SOUND_RX.captures_iter(txt) {
//         let name = cap[2].to_string();
//         let url = cap[1].to_string();
//         sounds.push(Sound { name, url });
//     }
//     sounds
// }

#[derive(Debug, Deserialize)]
struct ServerSound {
    name: String,
    path: String,
    play_count: i64,
}

#[derive(Debug, Serialize)]
struct PlaySoundPayload {
    name: String,
}

pub fn parse_sounds(txt: &str) -> Vec<Sound> {
    let sounds: Vec<ServerSound> = serde_json::from_str(txt).unwrap();
    sounds
        .into_iter()
        .map(|sound| Sound {
            name: sound.name,
            url: sound.path,
            play_count: sound.play_count,
        })
        .collect()
}

pub async fn get_sounds() -> Result<Vec<Sound>, gloo_net::Error> {
    let txt = get_sounds_strings().await?;
    let mut sounds = parse_sounds(&txt);
    sort_sounds(&mut sounds);
    Ok(sounds)
}

pub async fn play_sound(url: String) -> Result<(), gloo_net::Error> {
    let url = format!("{}/api/v1/play/{}", BASE_URL.as_str(), url);
    let req = RequestBuilder::new(&url)
        .method(Method::GET)
        .mode(RequestMode::Cors)
        .cache(RequestCache::NoCache)
        .build()?;

    req.send().await?;

    Ok(())
}

pub async fn kill_mplayer() -> Result<(), gloo_net::Error> {
    let req = RequestBuilder::new(&(&KILLALL_URL))
        .method(Method::GET)
        .cache(RequestCache::NoCache)
        .build()?;

    req.send().await?;

    Ok(())
}

fn sort_sounds(sounds: &mut Vec<Sound>) {
    // Sort alphabetically by name
    sounds.sort_by(|a, b| a.name.cmp(&b.name));

    // Sort by play count
    sounds.sort_by(|a, b| b.play_count.cmp(&a.play_count));

    let mut hl_sounds = Vec::new();

    // Move all sounds starting with "hl-sounds" to the end
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

// const TEST_TXT: &str = include_str!("../data/127.0.0.1.html");
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
