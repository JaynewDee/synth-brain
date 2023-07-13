use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct Commander;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ImageResponse {
    created: i32,
    data: ResponseData,
}

type ResponseData = Vec<DataItem>;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DataItem {
    url: String,
}

impl Commander {
    async fn generate_image(url: &str, api_key: &str, prompt: &str) -> ImageResponse {
        let auth = format!("Authorization: Bearer {}", api_key);

        let data = json!({
            "prompt": prompt,
            "n": 1,
            "size": "256x256"
        });

        let body = to_string(&data).expect("Failed to serialize JSON object ... ");

        let res = Command::new("curl")
            .arg(url)
            .args(["-H", "Content-Type: application/json"])
            .args(["-H", auth.as_str()])
            .args(["-d", &body])
            .output()
            .expect("Curl request failed ... ");

        if res.status.success() {
            println!("Engine returned a healthy response.");
        } else {
            let error = String::from_utf8_lossy(&res.stderr);
            eprintln!("ERROR!: {}", error);
        };

        serde_json::from_str(&String::from_utf8_lossy(&res.stdout)).unwrap()
    }

    async fn download_image(
        img_res: ImageResponse,
        out_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img_url = &img_res.data[0].url;

        let response = reqwest::get(img_url).await?;
        let mut file = File::create(out_name)?;

        let content = response.bytes().await?;

        file.write_all(&content)?;
        println!("Image downloaded successfully.");

        Ok(())
    }

    fn get_out_name(prompt: &str) -> String {
        let mut out_name = prompt.replace(" ", "_");
        out_name.push_str(".png");
        out_name
    }

    pub async fn generate_and_download(prompt: &str) {
        let api_key = env::var("API_KEY").unwrap();
        let url = "https://api.openai.com/v1/images/generations";
        let image_res = Self::generate_image(url, &api_key, prompt).await;
        let _ = Self::download_image(image_res, Self::get_out_name(prompt)).await;
    }
}
