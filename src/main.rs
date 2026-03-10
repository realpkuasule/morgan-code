use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;
use morgan_code::{
    config::Config,
    llm::LLMFactory,
    tools::ToolRegistry,
    agent::Agent,
    ui::Spinner,
    markdown::MarkdownRenderer,
};

#[derive(Parser)]
#[command(name = "morgan")]
#[command(about = "Morgan Code - AI coding assistant with customizable LLM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive chat session
    Chat,
    /// Initialize configuration file
    Init,
    /// Show current configuration
    Config,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> morgan_code::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            init_config()?;
        }
        Some(Commands::Config) => {
            show_config()?;
        }
        Some(Commands::Chat) | None => {
            start_chat().await?;
        }
    }

    Ok(())
}

fn init_config() -> morgan_code::Result<()> {
    let config_path = Config::config_path()?;

    if config_path.exists() {
        println!("Config file already exists at: {}", config_path.display());
        return Ok(());
    }

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let default_config = r#"[llm]
provider = "deepseek"
model = "deepseek-reasoner"
api_key_env = "DEEPSEEK_API_KEY"
temperature = 0.7
max_tokens = 4096

[llm.deepseek]
base_url = "https://api.deepseek.com/v1"

[llm.openai]
base_url = "https://api.openai.com/v1"

[tools]
enabled = ["read", "write", "edit", "glob", "grep", "shell"]
shell_timeout_seconds = 120

[agent]
max_iterations = 50
enable_background_tasks = true

[ui]
show_spinner = true
color_output = true
"#;

    std::fs::write(&config_path, default_config)?;
    println!("Created config file at: {}", config_path.display());
    println!("\nPlease set your API key in the environment:");
    println!("  export DEEPSEEK_API_KEY=your-api-key-here");
    println!("\nOr switch to OpenAI by editing the config file and setting:");
    println!("  export OPENAI_API_KEY=your-api-key-here");

    Ok(())
}

fn show_config() -> morgan_code::Result<()> {
    let config = Config::load()?;
    println!("Current configuration:");
    println!("  Provider: {}", config.llm.provider);
    println!("  Model: {}", config.llm.model);
    println!("  API Key Env: {}", config.llm.api_key_env);
    println!("  Max Iterations: {}", config.agent.max_iterations);
    println!("  Enabled Tools: {}", config.tools.enabled.join(", "));
    Ok(())
}

async fn start_chat() -> morgan_code::Result<()> {
    println!("Morgan Code - AI Coding Assistant");
    println!("Type 'exit' or 'quit' to end the session\n");

    let config = Config::load()?;
    let api_key = config.get_api_key()?;

    let llm = LLMFactory::create(&config.llm, api_key)?;
    let tools = Arc::new(ToolRegistry::new());
    let mut agent = Agent::new(llm, tools, config.agent.max_iterations);

    // Create markdown renderer
    let md_renderer = MarkdownRenderer::new();

    // Create rustyline editor with history support
    let mut rl = DefaultEditor::new()
        .map_err(|e| morgan_code::MorganError::Agent(format!("Failed to create editor: {}", e)))?;

    // Load command history if available
    let history_path = dirs::home_dir()
        .map(|h| h.join(".morgan-code").join("history.txt"));

    if let Some(ref path) = history_path {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = rl.load_history(path);
    }

    loop {
        // Print separator line before input
        println!("{}", "─".repeat(80));

        // Use rustyline to read input (without color in prompt to avoid duplication)
        let readline = rl.readline("❯ ");

        let input = match readline {
            Ok(line) => {
                // Add to history
                let _ = rl.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("Use 'exit' or 'quit' to end the session");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("\nGoodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        };

        let input = input.trim();

        if input.is_empty() {
            // Print separator line after empty input
            println!("{}", "─".repeat(80));
            continue;
        }

        // Move cursor up to overwrite the input line with colored version
        print!("\x1B[1A\x1B[2K"); // Move up one line and clear it

        // Display user input with white text on cyan background using direct ANSI codes
        print!("\x1b[97;46m❯ {}\x1b[0m\n", input); // 97=bright white, 46=cyan bg

        // Print separator line after input
        println!("{}", "─".repeat(80));

        if input == "exit" || input == "quit" {
            println!("\nGoodbye!");
            break;
        }

        if input == "clear" {
            agent.clear_context();
            println!("\nContext cleared.\n");
            continue;
        }

        let spinner = if config.ui.show_spinner {
            Some(Spinner::new("Thinking..."))
        } else {
            None
        };

        match agent.run(input.to_string()).await {
            Ok(response) => {
                if let Some(s) = spinner {
                    s.finish_and_clear();
                }
                // Ensure all styles are reset before printing response
                print!("\x1b[0m");
                println!("\nMorgan:");
                md_renderer.print(&response);
                println!();
            }
            Err(e) => {
                if let Some(s) = spinner {
                    s.finish_and_clear();
                }
                print!("\x1b[0m");
                eprintln!("\nError: {}\n", e);
            }
        }
    }

    // Save command history
    if let Some(ref path) = history_path {
        let _ = rl.save_history(path);
    }

    Ok(())
}
