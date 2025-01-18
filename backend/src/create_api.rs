use reqwest::Client;
use dotenv::dotenv;
use std::env;

pub async fn create_gemini_api_post(user_prompt: String) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
  
  dotenv().ok();

  let api_key = env::var("GEMINI_API").expect("GEMINI_API not found in .env");
  let base_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={api_key}");
  
  fn body_template_parser(user_prompt: String) -> String {
    let body_template = format!(r#"
      {{
        "contents": [{{
          "parts": [{{
            "text": "{}"
          }}]
        }}]
      }}
      "#, user_prompt);

    body_template
  }

  let client = Client::new();
  let res = client
    .post(&base_url)
    .header("Content-Type", "application/json")
    .body(body_template_parser(user_prompt))
    .send()
    .await?;
  Ok(res)
}