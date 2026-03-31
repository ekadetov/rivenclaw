mod agent;
mod context;
mod harness;
mod tools;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rivenclaw", about = "AI coding assistant harness")]
struct Cli {
    /// The prompt to run
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.prompt {
        Some(prompt) => {
            println!("prompt: {prompt}");
        }
        None => {
            println!("rivenclaw: no prompt provided");
        }
    }

    Ok(())
}
