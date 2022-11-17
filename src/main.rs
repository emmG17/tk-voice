use base64;
use reqwest;
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct TTSData {
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
    s_key: String,
    v_str: String,
}

fn create_url(voice: String, text: String) -> String {
    let base_url =
        "https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?".to_string();
    let mapping = "&speaker_map_type=0&aid=1233".to_string();

    // Concat arguments
    let voice_param = format!("text_speaker={}", voice);
    let text_param = format!("&req_text={}", text);
    let url = format!("{}{}{}{}", base_url, voice_param, text_param, mapping);
    return url;
}

async fn get_json(url: String, session_id: String) -> Result<TTSData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.post(url)
    .header(USER_AGENT, "com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)")
    .header("Cookie", format!("sessionid={}", session_id))
    .header(CONTENT_LENGTH, 0)
    .send()
    .await?
    .json::<TTSData>()
    .await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let default_voice = args[1].to_string();
    let default_text = args[2].to_string();
    let session_id = args[3].to_string();
    let file_name = args[4].to_string();
    let extension = ".mp3".to_string();
    let full_name = format!("{}{}", file_name, extension);

    let url = create_url(default_voice, default_text);

    if let Ok(json) = get_json(url, session_id).await {
        let data = json.data.v_str;
        let bytes = base64::decode(data).unwrap();
        let mut f = File::create(full_name).expect("Unable to create file");
        f.write_all(&bytes).expect("Unable to write data");
    } else {
        println!("Sad news");
    }

    Ok(())
}
