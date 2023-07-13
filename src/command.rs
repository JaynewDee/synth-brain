use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::consts;
pub struct Commander;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ImageResponse {
    created: i32,
    data: ResponseData,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DataItem {
    url: String,
}

type ResponseData = Vec<DataItem>;

impl Commander {
    async fn generate_image(prompt: &str) -> Result<ImageResponse, anyhow::Error> {
        let api_key = env::var("API_KEY")?;

        let auth_header = format!("Authorization: Bearer {}", api_key);

        // serde_json macro simplifies serialization of request body
        let data = json!({
            "prompt": prompt,
            "n": 1,
            "size": "256x256"
        });

        let body = to_string(&data)?;

        // No OpenAI Rust library, use std::process::Command to construct vanilla curl
        let res = Command::new("curl")
            .arg(consts::IMG_GEN_URL)
            .args(["-H", "Content-Type: application/json"])
            .args(["-H", auth_header.as_str()])
            .args(["-d", &body])
            .output()?;

        if res.status.success() {
            println!("Engine returned a healthy response.");
        } else {
            let error = String::from_utf8_lossy(&res.stderr);
            eprintln!("CURL ERROR!: {}", error);
        };

        let utf8 = &String::from_utf8_lossy(&res.stdout);
        let json = serde_json::from_str(&utf8)?;

        Ok(json)
    }

    async fn download_image(img_res: ImageResponse, out_name: String) -> Result<(), anyhow::Error> {
        let img_url = &img_res.data[0].url;

        let response = reqwest::get(img_url).await?;
        let mut file = File::create(out_name)?;

        let content = response.bytes().await?;

        file.write_all(&content)?;
        println!("Image downloaded successfully.");

        Ok(())
    }

    fn get_out_name(prompt: &str) -> String {
        let mut out_name = prompt.replace(' ', "_");
        out_name.push_str(".png");
        out_name
    }

    pub async fn generate_and_download(prompt: &str) -> Result<(), anyhow::Error> {
        let image_res = Self::generate_image(prompt).await?;
        let _ = Self::download_image(image_res, Self::get_out_name(prompt)).await?;
        Ok(())
    }
}
