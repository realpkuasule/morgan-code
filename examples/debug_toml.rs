fn main() {
    let repl_toml = r#"
[ui]
mode = "repl"
"#;
    
    println!("Original TOML:");
    println!("{}", repl_toml);
    println!();
    
    // Parse as generic TOML value
    let parsed: toml::Value = toml::from_str(repl_toml).unwrap();
    println!("Parsed as TOML Value: {:#}", parsed);
    println!();
    
    // Get the ui table
    if let Some(ui_table) = parsed.get("ui") {
        println!("UI table: {:#}", ui_table);
        println!();
        
        if let Some(mode_value) = ui_table.get("mode") {
            println!("Mode value: {:?}", mode_value);
            println!("Mode value type: {:?}", mode_value.type_str());
        }
    }
}
