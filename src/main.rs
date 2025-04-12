use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use toolius_maximus::{Message, MessageType};
use toolius_maximus::{RequestBody, API};
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // Load .env
    let token = env::var("GITHUB_TOKEN")?;

    let url = "https://models.inference.ai.azure.com/chat/completions";
    let client = API::new(url.to_string(), token.to_string());

    let system_message = Message::new(MessageType::System, "You are a helpful assistant.");

    let mut conversation: Vec<Message> = vec![
        system_message.clone()
    ];

    println!("Enter your message | 'q' to quit");
    println!("{}", system_message);
    loop {
        // Ask for user input
        print!("{}", "[User]: ".bold().magenta());
        io::stdout().flush()?;
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
        let body = RequestBody::new(
            conversation
                .iter()
                .map(|m| m.json())
                .collect(),
            0.7,
            1.0,
            100,
            "gpt-4o-mini".to_string(),
        );

        let response = client.send(body).await?;

        // Parse and display assistant response
        if let Some(choices) = response.get("choices").and_then(|c| c.as_array()) {

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
            println!("Unexpected response format: {:?}", response);
        }
    }

    Ok(())
}

