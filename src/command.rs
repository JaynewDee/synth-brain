use crate::models::CompletionChoice;
use crate::{consts, models};
use models::{ImageResponse, MessageItem, TextResponse};

use serde_json::{json, to_string};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

enum Medium {
    Text,
    Image,
}

impl Medium {}
pub struct Commander;

impl Commander {
    fn generate_image(prompt: &str) -> Result<ImageResponse, anyhow::Error> {
        let api_key = env::var("API_KEY")?;

        let auth_header = format!("Authorization: Bearer {api_key}");

        // serde_json macro simplifies serialization of request body
        let data = json!({
            "prompt": prompt,
            "n": 1,
            "size": "256x256",
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

    fn format_out_name(prompt: &str, ext: &str) -> String {
        let mut out_name = prompt.replace(' ', "_");
        out_name.push_str(ext);
        out_name
    }

    pub async fn generate_and_download(prompt: &str) -> Result<(), anyhow::Error> {
        let image_res = Self::generate_image(prompt)?;
        let _ = Self::download_image(image_res, Self::format_out_name(prompt, ".png")).await?;
        Ok(())
    }

    async fn text_completion(prompt: &str) -> Result<TextResponse, anyhow::Error> {
        let api_key = env::var("API_KEY")?;

        let auth_header = format!("Authorization: Bearer {}", api_key);

        let messages = vec![MessageItem {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        let data = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
            "temperature": 0.7
        });

        let body = to_string(&data)?;

        let res = Command::new("curl")
            .arg(consts::TXT_GEN_URL)
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

        let from_utf8 = &String::from_utf8_lossy(&res.stdout);
        let json = serde_json::from_str(&from_utf8)?;

        Ok(json)
    }

    async fn write_responses(
        responses: Vec<CompletionChoice>,
        prompt: &str,
    ) -> Result<(), anyhow::Error> {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("completion_responses.txt")?;

        file.write_all(format!("user ::: {}\n", prompt).as_bytes())?;

        let templatize = move |role, content| format!(r"{} ::: {}", role, content);

        for choice in responses.iter() {
            let template = templatize(&choice.message.role, &choice.message.content);
            file.write_all(template.as_bytes())?;
        }

        Ok(())
    }

    pub async fn complete_and_write(prompt: &str) -> Result<(), anyhow::Error> {
        let text_res = Self::text_completion(prompt).await?;
        let responses = text_res.choices;
        let _ = Self::write_responses(responses, prompt).await?;
        Ok(())
    }
}
