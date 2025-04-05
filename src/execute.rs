use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Local};
use clearscreen::clear;
use colored::Colorize;

use crate::execute2::execute2;

fn substitute_commands(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'(') {
            chars.next(); // Consume '('
            let mut command = String::new();
            let mut depth = 1;

            while let Some(&next) = chars.peek() {
                chars.next();
                if next == '(' {
                    depth += 1;
                } else if next == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                command.push(next);
            }

            if let Ok(output_str) = execute_command(&command) {
                output.push_str(&output_str);
            }
        } else {
            output.push(ch);
        }
    }

    output
}

fn execute_command(command: &str) -> Result<String, String> {
    use std::process::Command;

    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn expand_variables(input: &str) -> String {
    let substituted = substitute_commands(input);
    let mut output = String::new();
    let mut chars = substituted.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            let mut var_name = String::new();
            while let Some(&next) = chars.peek() {
                if next.is_alphanumeric() || next == '_' {
                    var_name.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if let Ok(val) = env::var(&var_name) {
                output.push_str(&val);
            } else {
                output.push('$');
                output.push_str(&var_name);
            }
        } else {
            output.push(ch);
        }
    }

    output
}

fn execute_piped_commands(input: &str) {
    let commands: Vec<&str> = input.split('|').map(|s| s.trim()).collect();
    if commands.is_empty() {
        return;
    }

    let mut previous_output: Option<std::process::ChildStdout> = None;

    for (i, command) in commands.iter().enumerate() {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mut cmd = Command::new(parts[0]);
        cmd.args(&parts[1..]);

        if let Some(output) = previous_output.take() {
            cmd.stdin(Stdio::from(output));
        }

        if i < commands.len() - 1 {
            cmd.stdout(Stdio::piped());
        }

        let mut output = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Error executing command '{}': {}", command, e);
                return;
            }
        };

        if i < commands.len() - 1 {
            previous_output = output.stdout.take();
        } else {
            let _ = output.wait_with_output().map(|output| {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            });
        }
    }
}

pub fn execute(raw_input: &str) {
    let input = expand_variables(raw_input);

    if input.contains('|') {
        execute_piped_commands(&input);
        return;
    }

    let mut parts = input.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args: Vec<&str> = parts.collect();

    match cmd {
        "cd" => {
            let default_home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
            let new_dir = args.get(0).map(|s| *s).unwrap_or(&default_home);
            let path = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(path) {
                eprintln!("cd: {}", e);
            }
        }

        "ls" | "la" => {
            let show_hidden = cmd == "la";
            let target_dir = args
                .get(0)
                .map(|s| {
                    if s.starts_with('~') {
                        s.replacen('~', &env::var("HOME").unwrap_or("/".into()), 1)
                    } else {
                        s.to_string()
                    }
                })
                .unwrap_or_else(|| ".".to_string());

            match fs::read_dir(&target_dir) {
                Ok(entries) => {
                    let mut files = vec![];

                    for entry in entries.flatten() {
                        let path = entry.path();
                        let metadata = match entry.metadata() {
                            Ok(m) => m,
                            Err(_) => continue,
                        };
                        let name = entry.file_name().to_string_lossy().to_string();
                        if !show_hidden && name.starts_with('.') {
                            continue;
                        }

                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        let file_type = if metadata.is_dir() {
                            "[DIR]".blue().bold()
                        } else if metadata.permissions().mode() & 0o111 != 0 {
                            "[EXE]".green().bold()
                        } else if ["mp4", "mkv", "webm"].contains(&ext.as_str()) {
                            "[VIDEO]".bright_red()
                        } else if ["mp3", "wav", "flac"].contains(&ext.as_str()) {
                            "[AUDIO]".bright_cyan()
                        } else if ["png", "jpg", "jpeg", "webp", "gif", "bmp", "tiff", "svg"]
                            .contains(&ext.as_str())
                        {
                            "[IMG]".magenta()
                        } else if ext == "rs" {
                            "[RUST]".truecolor(255, 100, 50)
                        } else if ext == "py" {
                            "[PY]".yellow()
                        } else if ["html", "htm"].contains(&ext.as_str()) {
                            "[HTML]".bright_red()
                        } else if ext == "htmx" {
                            "[HTMX]".truecolor(255, 80, 80)
                        } else if ["css", "scss", "sass"].contains(&ext.as_str()) {
                            "[CSS]".truecolor(255, 105, 180)
                        } else if ["js", "ts", "mjs", "jsx", "tsx"].contains(&ext.as_str()) {
                            "[JS/TS]".yellow()
                        } else if ext == "json" {
                            "[JSON]".bright_yellow()
                        } else if ["yaml", "yml"].contains(&ext.as_str()) {
                            "[YAML]".bright_white()
                        } else if ext == "lock" {
                            "[LOCK]".bright_magenta()
                        } else if ext == "toml" {
                            "[TOML]".bright_blue()
                        } else if ["txt", "md", "rst", "norg"].contains(&ext.as_str()) {
                            "[TEXT]".white()
                        } else if ext == "c" {
                            "[C]".cyan()
                        } else if ["cpp", "cc", "cxx", "h", "hpp"].contains(&ext.as_str()) {
                            "[C++]".bright_cyan()
                        } else if ext == "cs" {
                            "[C#]".purple()
                        } else if ext == "java" {
                            "[JAVA]".red()
                        } else if ["kt", "kts"].contains(&ext.as_str()) {
                            "[KOTLIN]".truecolor(255, 100, 200)
                        } else if ext == "go" {
                            "[GO]".cyan()
                        } else if ext == "rb" {
                            "[RUBY]".red()
                        } else if ext == "php" {
                            "[PHP]".magenta()
                        } else if ext == "swift" {
                            "[SWIFT]".bright_white()
                        } else if ext == "lua" {
                            "[LUA]".blue()
                        } else if ["sh", "bash", "zsh"].contains(&ext.as_str()) {
                            "[SHELL]".bright_green()
                        } else if ["asm", "s"].contains(&ext.as_str()) {
                            "[ASM]".truecolor(180, 180, 180)
                        } else if ext == "wasm" {
                            "[WASM]".bright_blue()
                        } else if ["zip", "tar", "gz", "xz", "7z", "rar"].contains(&ext.as_str()) {
                            "[ARCH]".bright_yellow()
                        } else if ext == "pdf" {
                            "[PDF]".red()
                        } else if ext == "log" {
                            "[LOG]".truecolor(128, 128, 128)
                        } else if ext == "exe" {
                            "[EXE]".green()
                        } else if ext == "msi" {
                            "[INSTALL]".truecolor(255, 180, 0)
                        } else if ext == "apk" {
                            "[APK]".truecolor(80, 220, 80)
                        } else if ext == "iso" {
                            "[ISO]".truecolor(220, 220, 255)
                        } else if ["db", "sqlite", "sql"].contains(&ext.as_str()) {
                            "[DB]".truecolor(255, 100, 255)
                        } else {
                            "[FILE]".dimmed()
                        };

                        fn format_size(bytes: u64) -> String {
                            const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
                            let mut size = bytes as f64;
                            let mut unit = 0;
                            while size >= 1024.0 && unit < UNITS.len() - 1 {
                                size /= 1024.0;
                                unit += 1;
                            }
                            format!("{:.1} {}", size, UNITS[unit])
                        }

                        let size = format_size(metadata.len());
                        let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
                        let datetime: DateTime<Local> = modified.into();
                        let formatted_time = datetime.format("%Y-%m-%d %H:%M").to_string();

                        files.push((name, file_type.to_string(), size, formatted_time));
                    }

                    files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

                    println!(
                        "\n{} {}",
                        "📁".bold(),
                        target_dir
                            .replace(&env::var("HOME").unwrap_or_default(), "~")
                            .bright_white()
                            .bold()
                    );
                    let count = files.len();

                    for (i, (name, ftype, size, time)) in files.into_iter().enumerate() {
                        let is_last = i == count - 1;
                        let branch = if is_last { "└──" } else { "├──" };
                        let pipe = if is_last { "    " } else { "│   " };

                        println!("{} {}", branch, name.bright_white().bold());
                        println!(
                            "{} {}   {}   {}",
                            pipe,
                            size.bright_green(),
                            ftype,
                            time.dimmed()
                        );
                    }
                }
                Err(e) => eprintln!("{}", format!("ls: {}", e).red()),
            }
        }

        "clear" => {
            clear().unwrap();
        }
        "echo" => {
            println!("{}", args.join(" "));
        }

        _ => {
            execute2(&input);
        }
    }
}
