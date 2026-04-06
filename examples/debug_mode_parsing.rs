use morgan_code::config::UIConfig;

fn main() {
    println!("Debugging mode parsing...\n");
    
    // Test with repl mode
    let repl_toml = r#"
[ui]
mode = "repl"
"#;
    
    println!("TOML content:");
    println!("{}", repl_toml);
    println!();
    
    // Parse as TOML value first
    let parsed: toml::Value = toml::from_str(repl_toml).unwrap();
    println!("Parsed TOML: {:?}", parsed);
    println!();
    
    // Try to parse as UIConfig
    match toml::from_str::<UIConfig>(repl_toml) {
        Ok(config) => {
            println!("Successfully parsed as UIConfig:");
            println!("  mode: {:?}", config.mode);
        },
        Err(e) => {
            println!("Failed to parse: {}", e);
        }
    }
}
