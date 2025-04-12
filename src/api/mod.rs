use reqwest::Client;
use serde_json::Value;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RequestBody {
    pub messages: Vec<Value>,
    pub temperature: f64,
    pub top_p: f64,
    pub max_tokens: i64,
    pub model: String,
}

impl RequestBody {
    pub fn new(messages: Vec<Value>, temperature: f64, top_p: f64, max_tokens: i64, model: String) -> Self {
        Self {
            messages,
            temperature,
            top_p,
            max_tokens,
            model,
        }
    }
}

pub struct API {
    pub client: Client,
    pub url: String,
    pub token: String
}

impl API {
    pub fn new(url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            url,
            token
        }
    }

    pub async fn send(&self, body: RequestBody) -> Result<Value, reqwest::Error> {
        let response = self.client
            .post(&self.url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.token))
                .json(&body)
                .send()
                .await?;

        let json: Value = response.json().await?;
        Ok(json)
    }
}
