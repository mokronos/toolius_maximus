use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use toolius_maximus::Message;
use toolius_maximus::Client;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    dotenv().ok();
    let token = env::var("GITHUB_TOKEN")?;

    let url = "https://models.inference.ai.azure.com/chat/completions";
    let client = Client::new(url.to_string(), token.to_string());

    let system_message = Message::system("You are a helpful assistant".to_string());

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

        let user_message = Message::user(user_input.to_string());

        // Clear the last line (user input prompt + input)
        print!("\x1b[1A\x1b[2K"); // move up + clear line
        io::stdout().flush()?;
        println!("{}", user_message);

        conversation.push(user_message);

        let response = client.create("gpt-4o-mini".to_string(), conversation.clone()).await?;

        // Parse and display assistant response
        if let Some(choices) = response.get("choices").and_then(|c| c.as_array()) {

            // Add assistant's response to the conversation
            if let Some(content) = choices[0]
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_str())
            {
                let agent_message = Message::agent(content.to_string());
                println!("{}", agent_message);
                conversation.push(agent_message);
            }
        } else {
            println!("Unexpected response format: {:?}", response);
        }
    }

    Ok(())
}

