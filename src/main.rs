use reqwest;
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
//use serde::Deserialize;

fn create_url(voice: String, text: String) -> String {
    let base_url =
        "https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?".to_string();
    let mapping = "&speaker_map_type=0&aid=1233".to_string();

    // Concat arguments
    let voice_param = format!("text_speaker={}", voice);
    let text_param = format!("&req_text={}", text);
    return format!("{}{}{}{}", base_url, voice_param, text_param, mapping);
}

async fn get_json(url: String, session_id: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.post(url)
    .header(USER_AGENT, "com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)")
    .header("Cookie", format!("sessionid={}", session_id))
    .header(CONTENT_LENGTH, 0)
    .send()
    .await?
    .text()
    .await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let default_voice: String = "es_mx_002".to_string();
    let default_text: String = "hola".to_string();
    let session_id: String = "1234562134".to_string();
    let url = create_url(default_voice, default_text);
    println!("{}", url);
    let body = get_json(url, session_id).await;
    println!("{:#?}", body);
}
