use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env
    let token = env::var("GITHUB_TOKEN")?;

    let url = "https://models.inference.ai.azure.com/chat/completions";
    let client = Client::new();

    let mut conversation: Vec<Value> = vec![
        json!({
            "role": "system",
            "content": "You are a helpful assistant."
        })
    ];

    loop {
        // Ask for user input
        print!("You: ");
        io::stdout().flush()?; // Ensure prompt is printed
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        // Exit condition
        if user_input.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        // Add user input to conversation
        conversation.push(json!({
            "role": "user",
            "content": user_input
        }));

        // Send the conversation to the API
        let body = json!({
            "messages": conversation,
            "temperature": 1.0,
            "top_p": 1.0,
            "max_tokens": 1000,
            "model": "gpt-4o-mini"
        });

        let res = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await?;

        let json: Value = res.json().await?;

        // Parse and display assistant response
        if let Some(choices) = json.get("choices").and_then(|c| c.as_array()) {
            for choice in choices {
                if let Some(role) = choice.get("message").and_then(|m| m.get("role")) {
                    if let Some(content) = choice.get("message").and_then(|m| m.get("content")) {
                        println!("{}: {}", role.as_str().unwrap_or("?"), content.as_str().unwrap_or(""));
                    }
                }
            }

            // Add assistant's response to the conversation
            if let Some(content) = choices[0]
                .get("message")
                .and_then(|m| m.get("content"))
            {
                conversation.push(json!({
                    "role": "assistant",
                    "content": content
                }));
            }
        } else {
            println!("Unexpected response format: {:?}", json);
        }
    }

    Ok(())
}

