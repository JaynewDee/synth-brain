use crate::models::CompletionChoice;
use crate::{consts, models};
use models::{ImageResponse, MessageItem, TextResponse};

use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use serde_json::{json, to_string};

pub struct Operation;

impl Operation {
    fn auth() -> Result<String, anyhow::Error> {
        let api_key = env::var("API_KEY")?;

        Ok(format!("Authorization: Bearer {api_key}"))
    }

    fn generate_image(prompt: &str) -> Result<ImageResponse, anyhow::Error> {
        let auth_header = match Self::auth() {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let data = json!({
            "prompt": prompt,
            "n": 1,
            "size": "256x256",
        });

        let body = to_string(&data)?;

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
        let out_name = Self::format_out_name(prompt, ".png");
        Self::download_image(image_res, out_name).await?;
        Ok(())
    }

    fn text_completion(prompt: &str) -> Result<TextResponse, anyhow::Error> {
        let auth_header = match Self::auth() {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let messages = vec![MessageItem::new("user".to_string(), prompt.to_string())];

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

    fn write_responses(
        responses: Vec<CompletionChoice>,
        prompt: &str,
    ) -> Result<(), anyhow::Error> {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("completion_responses.txt")?;

        file.write_all(format!("\nuser ::: {}\n", prompt).as_bytes())?;

        for choice in responses.iter() {
            let (role, content) = (&choice.message.role, &choice.message.content);

            let template = format!("{role} ::: {content}\n");
            file.write_all(template.as_bytes())?;
        }

        Ok(())
    }

    pub async fn complete_and_write(prompt: &str) -> Result<(), anyhow::Error> {
        let text_res = Self::text_completion(prompt)?;
        let responses = text_res.choices;
        let _ = Self::write_responses(responses, prompt)?;
        Ok(())
    }

    pub fn speech_to_text(filepath: &str) -> Result<(), anyhow::Error> {
        let auth_header = match Self::auth() {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let file_header = ["--form", &format!("file=@{filepath}")];

        let res = Command::new("curl")
            .args(["--request", "POST"])
            .args(["--url", consts::SPEECH_URL])
            .args(["--header", "Content-Type: multipart/form-data"])
            .args(["--header", auth_header.as_str()])
            .args(file_header)
            .args(["--form", "model=whisper-1"])
            .args(["--form", "response_format=text"])
            .output()?;

        if res.status.success() {
            println!("Engine returned a healthy response.");
        } else {
            let error = String::from_utf8_lossy(&res.stderr);
            eprintln!("CURL ERROR!: {}", error);
        };

        let translated_text = String::from_utf8_lossy(&res.stdout);

        println!("{}", translated_text);

        Ok(())
    }
}
