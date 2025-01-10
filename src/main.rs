use colored::*;
use serde_json::Value;
use std::io::{self, BufRead};

fn colorize_json_compact(json: &Value) -> String {
    match json {
        Value::Object(map) => {
            let contents: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "\"{}\": {}",
                        k.bright_blue().bold(),
                        colorize_json_compact(v)
                    )
                })
                .collect();
            format!("{{{}}}", contents.join(", "))
        }
        Value::Array(arr) => {
            let contents: Vec<String> = arr.iter().map(|v| colorize_json_compact(v)).collect();
            format!("[{}]", contents.join(", "))
        }
        // Other value types remain the same
        Value::String(s) => format!("\"{}\"", s.bright_green()),
        Value::Number(n) => n.to_string().bright_yellow().to_string(),
        Value::Bool(b) => b.to_string().bright_purple().to_string(),
        Value::Null => "null".bright_red().to_string(),
    }
}

fn colorize_json(json: &Value, nesting_level: usize) -> String {
    if nesting_level > 1 {
        // If nesting level > 2, switch to  compact formatting
        return colorize_json_compact(json);
    }
    let indent = nesting_level * 2;

    // Original pretty formatting for nesting level <= 2
    match json {
        Value::Object(map) => {
            let indent_str = " ".repeat(indent);
            let next_indent = " ".repeat(indent + 2);

            let contents: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}\"{}\": {}",
                        next_indent,
                        k.bright_blue().bold(),
                        colorize_json(v, nesting_level + 1)
                    )
                })
                .collect();

            format!("{{\n{}\n{}}}", contents.join(",\n"), indent_str)
        }
        Value::Array(arr) => {
            let indent_str = " ".repeat(indent);
            let next_indent = " ".repeat(indent + 2);

            let contents: Vec<String> = arr
                .iter()
                .map(|v| format!("{}{}", next_indent, colorize_json(v, nesting_level + 1)))
                .collect();

            format!("[\n{}\n{}]", contents.join(",\n"), indent_str)
        }
        _ => colorize_json_compact(json),
    }
}

fn process_line(line: &str) {
    let trimmed = line.trim();

    // Check if the line starts with { or [
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        // Try to parse as JSON
        match serde_json::from_str::<Value>(trimmed) {
            Ok(json) => {
                // Successfully parsed as JSON, colorize it
                println!("{}", colorize_json(&json, 0));
                return;
            }
            Err(_) => {}
        }
    }

    // Not JSON or failed to parse, colorize as plain text
    println!("{}", trimmed.bright_white());
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(line) => process_line(&line),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}
