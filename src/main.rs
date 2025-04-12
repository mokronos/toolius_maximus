use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};
use toolius_maximus::{Message, MessageType};
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // Load .env
    let token = env::var("GITHUB_TOKEN")?;

    let url = "https://models.inference.ai.azure.com/chat/completions";
    let client = Client::new();

    let system_message = Message::new(MessageType::System, "You are a helpful assistant.");

    let mut conversation: Vec<Message> = vec![
        system_message
    ];

    println!("Enter your message | 'q' to quit");
    loop {
        // Ask for user input
        print!("{}", "[User]: ".bold().magenta());
        io::stdout().flush()?; // Ensure prompt is printed
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        // Exit condition
        if user_input.to_lowercase() == "exit" || user_input.to_lowercase() == "quit"|| user_input.to_lowercase() == "q" {
            println!("Goodbye!");
            break;
        }

        let user_message = Message::new(MessageType::User, user_input);

        conversation.push(user_message);

        // Send the conversation to the API
        let body = json!({
            "messages": conversation.iter().map(|m| m.clone().json()).collect::<Vec<Value>>(),
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

            // Add assistant's response to the conversation
            if let Some(content) = choices[0]
                .get("message")
                .and_then(|m| m.get("content"))
            {
                let agent_message = Message::new(MessageType::Agent, &content.to_string());
                println!("{}", agent_message);
                conversation.push(agent_message);
            }
        } else {
            println!("Unexpected response format: {:?}", json);
        }
    }

    Ok(())
}

