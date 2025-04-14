use log::debug;
use reqwest::Client as reqwestClient;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::messages::Message;

pub struct Client {
    pub client: reqwestClient,
    pub url: String,
    pub token: String,
}

impl Client {
    pub fn new(url: String, api_key: String) -> Self {
        Self {
            client: reqwestClient::new(),
            url,
            token: api_key,
        }
    }

    pub async fn create(
        &self,
        model: String,
        messages: Vec<Message>,
    ) -> Result<Value, reqwest::Error> {
        let body = ChatCompletionRequest::new(model, messages);
        debug!("Body: {}", serde_json::to_string_pretty(&body).unwrap());
        let response = self
            .client
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

#[derive(Serialize, Debug)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,

    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatCompletionRequest {
    pub fn new(model: String, messages: Vec<Message>) -> Self {
        Self {
            model,
            messages,
            tools: None,
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            stop: None,
            max_completion_tokens: None,
            presence_penalty: Some(0.0),
            frequency_penalty: Some(0.0),
            logit_bias: None,
            user: None,
        }
    }
}
