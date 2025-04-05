use execute::execute;
use hostname;
use rustyline::completion::{Completer, Pair};
use rustyline::config::Builder as ConfigBuilder;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Editor, Helper, hint::Hinter};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub mod execute;
pub mod execute2;
pub mod execute3;
pub mod xinsays;

#[derive(Serialize, Deserialize)]
struct XinuxConfig {
    prompt_style: String,
    aliases: HashMap<String, String>,
    autostart_commands: Vec<String>, // New field for autostart commands
}

impl Default for XinuxConfig {
    fn default() -> Self {
        Self {
            prompt_style: "single_line".into(),
            aliases: HashMap::new(),
            autostart_commands: Vec::new(), // Default to an empty list
        }
    }
}

fn xinux_dir() -> PathBuf {
    let dir = dirs::home_dir().unwrap().join(".config/xinux");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("Failed to create Xinux configuration directory.");
        println!("Created configuration directory at: {}", dir.display());
    }
    dir
}

fn config_file_path() -> PathBuf {
    xinux_dir().join("config.toml")
}

fn history_file_path() -> PathBuf {
    xinux_dir().join("history.txt")
}

fn load_or_create_config() -> XinuxConfig {
    let config_path = config_file_path();
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).expect("Failed to read config.");
        toml::from_str(&content).unwrap_or_default()
    } else {
        println!("\n\x1b[1;38;5;214m🌌 XinuxOS First-Time Setup\x1b[0m");
        let prompt_options = vec![
            (
                "single_line",
                "Single-line",
                "\x1b[38;5;214m🌌 [~] > \x1b[0m",
            ),
            ("two_line", "Two-line", "\x1b[38;5;214m📁 ~\n🌌 > \x1b[0m"),
            ("boxy", "Boxy", "\x1b[38;5;214m┌─[~]\n└─> \x1b[0m"),
            ("minimal", "Minimal", "\x1b[38;5;214m> \x1b[0m"),
            ("classic", "Classic", "\x1b[38;5;172mλ ~ > \x1b[0m"),
            (
                "bold_frame",
                "Bold Frame",
                "\x1b[38;5;214m╭─[~]\n╰─λ > \x1b[0m",
            ),
            ("arrowed", "Arrowed", "\x1b[38;5;214m➜ ~ \x1b[0m"),
            (
                "fancy_duo",
                "Fancy Duo",
                "\x1b[38;5;214m┃📁 ~\n┃🌌 > \x1b[0m",
            ),
            ("thin_line", "Thin Line", "\x1b[38;5;214m───── ~\n➤ \x1b[0m"),
            (
                "modern_box",
                "Modern Box",
                "\x1b[38;5;214m╔═[~]\n╚═➤ \x1b[0m",
            ),
            ("bracketed", "Bracketed", "\x1b[38;5;214m[~] λ ➤ \x1b[0m"),
            (
                "shell_style",
                "Shell Style",
                "\x1b[38;5;214mxinux@🌌:~$ \x1b[0m",
            ),
            (
                "bold_double",
                "Bold Double",
                "\x1b[38;5;214m╔═╦═[~]\n╚═╩═➤ \x1b[0m",
            ),
            (
                "bold_rounded",
                "Bold Rounded",
                "\x1b[38;5;214m╭─╮ ~\n╰─╯ ➤ \x1b[0m",
            ),
            (
                "bold_ascii",
                "Bold ASCII",
                "\x1b[38;5;214m[+++ ~ +++]\n[     ➤ \x1b[0m",
            ),
            (
                "bold_neon",
                "Bold Neon",
                "\x1b[38;5;123m███ ~\n▀▀▀ ➤ \x1b[0m",
            ),
            (
                "bold_circuit",
                "Circuit Board",
                "\x1b[38;5;226m┌─⏁─[~]\n└─⏚─➤ \x1b[0m".trim(),
            ),
            ("cyberpunk", "Cyberpunk", "\x1b[38;5;129m⟦~⟧\n⏁ \x1b[0m"),
            (
                "retro_green",
                "Retro Green",
                "\x1b[38;5;22m>----[~]\n\\--> \x1b[0m",
            ),
            (
                "space_age",
                "Space Age",
                "\x1b[38;5;117m🚀 ~\n>---> \x1b[0m",
            ),
            ("hacker", "Hacker", "\x1b[38;5;46m[[~]]\n# \x1b[0m"),
            ("pixel_art", "Pixel Art", "\x1b[38;5;202m⡇~⡇\n⠋ \x1b[0m"),
            ("matrix", "Matrix", "\x1b[38;5;46m⟦~⟧\n⏁ \x1b[0m"),
            (
                "steampunk",
                "Steampunk",
                "\x1b[38;5;94m⚙─[~]\n\\/─➤ \x1b[0m",
            ),
            ("ocean_wave", "Ocean Wave", "\x1b[38;5;38m~ ~ ~\n» \x1b[0m"),
            ("fire", "Fire", "\x1b[38;5;202m~*\n» \x1b[0m"),
            ("neon_sign", "Neon Sign", "\x1b[38;5;123m~*\n» \x1b[0m"),
            ("robot", "Robot", "\x1b[38;5;240m[~]\n🤖 \x1b[0m"),
            ("terminal", "Terminal", "\x1b[38;5;10m~$\n> \x1b[0m"),
        ];

        for (i, (_, name, preview)) in prompt_options.iter().enumerate() {
            println!("{i}. {name}:\n   {preview}\n");
        }

        let style = loop {
            print!(
                "Select your prompt style [0-{}]: ",
                prompt_options.len() - 1
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if let Ok(idx) = input.trim().parse::<usize>() {
                if idx < prompt_options.len() {
                    break prompt_options[idx].0.to_string();
                }
            }
            println!("Invalid selection. Try again.");
        };

        let config = XinuxConfig {
            prompt_style: style.clone(),
            ..Default::default()
        };

        fs::write(&config_path, toml::to_string(&config).unwrap())
            .expect("Could not write config");

        println!(
            "\n\x1b[32m✔ Configuration saved to {}\x1b[0m",
            config_path.display()
        );
        config
    }
}

fn reselect_prompt_style() {
    let prompt_options = vec![
        (
            "single_line",
            "Single-line",
            "\x1b[38;5;214m🌌 [~] > \x1b[0m",
        ),
        ("two_line", "Two-line", "\x1b[38;5;214m📁 ~\n🌌 > \x1b[0m"),
        ("boxy", "Boxy", "\x1b[38;5;214m┌─[~]\n└─> \x1b[0m"),
        ("minimal", "Minimal", "\x1b[38;5;214m> \x1b[0m"),
        ("classic", "Classic", "\x1b[38;5;172mλ ~ > \x1b[0m"),
        (
            "bold_frame",
            "Bold Frame",
            "\x1b[38;5;214m╭─[~]\n╰─λ \x1b[0m",
        ),
        ("arrowed", "Arrowed", "\x1b[38;5;214m➜ ~ \x1b[0m"),
        (
            "fancy_duo",
            "Fancy Duo",
            "\x1b[38;5;214m┃📁 ~\n┃🌌 > \x1b[0m",
        ),
        ("thin_line", "Thin Line", "\x1b[38;5;214m───── ~\n➤ \x1b[0m"),
        (
            "modern_box",
            "Modern Box",
            "\x1b[38;5;214m╔═[~]\n╚═➤ \x1b[0m",
        ),
        ("bracketed", "Bracketed", "\x1b[38;5;214m[~] λ ➤ \x1b[0m"),
        (
            "shell_style",
            "Shell Style",
            "\x1b[38;5;214mxinux@🌌:~$ \x1b[0m",
        ),
        (
            "bold_double",
            "Bold Double",
            "\x1b[38;5;214m╔═╦═[~]\n╚═╩═➤ \x1b[0m",
        ),
        (
            "bold_rounded",
            "Bold Rounded",
            "\x1b[38;5;214m╭─╮ ~\n╰─╯ ➤ \x1b[0m",
        ),
        (
            "bold_ascii",
            "Bold ASCII",
            "\x1b[38;5;214m[+++ ~ +++]\n[     ➤ \x1b[0m",
        ),
        (
            "bold_neon",
            "Bold Neon",
            "\x1b[38;5;123m███ ~\n▀▀▀ ➤ \x1b[0m",
        ),
        (
            "bold_circuit",
            "Circuit Board",
            "\x1b[38;5;226m┌─⏁─[~]\n└─⏚─➤ \x1b[0m".trim(),
        ),
        ("cyberpunk", "Cyberpunk", "\x1b[38;5;129m⟦~⟧\n⏁ \x1b[0m"),
        (
            "retro_green",
            "Retro Green",
            "\x1b[38;5;22m>----[~]\n\\--> \x1b[0m",
        ),
        (
            "space_age",
            "Space Age",
            "\x1b[38;5;117m🚀 ~\n>---> \x1b[0m",
        ),
        ("hacker", "Hacker", "\x1b[38;5;46m[[~]]\n# \x1b[0m"),
        ("pixel_art", "Pixel Art", "\x1b[38;5;202m⡇~⡇\n⠋ \x1b[0m"),
        ("matrix", "Matrix", "\x1b[38;5;46m⟦~⟧\n⏁ \x1b[0m"),
        (
            "steampunk",
            "Steampunk",
            "\x1b[38;5;94m⚙─[~]\n\\/─➤ \x1b[0m",
        ),
        ("ocean_wave", "Ocean Wave", "\x1b[38;5;38m~ ~ ~\n» \x1b[0m"),
        ("fire", "Fire", "\x1b[38;5;202m~*\n» \x1b[0m"),
        ("neon_sign", "Neon Sign", "\x1b[38;5;123m~*\n» \x1b[0m"),
        ("robot", "Robot", "\x1b[38;5;240m[~]\n🤖 \x1b[0m"),
        ("terminal", "Terminal", "\x1b[38;5;10m~$\n> \x1b[0m"),
    ];

    for (i, (_, name, preview)) in prompt_options.iter().enumerate() {
        println!("{i}. {name}:\n   {preview}\n");
    }

    let style = loop {
        print!(
            "Select your prompt style [0-{}]: ",
            prompt_options.len() - 1
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(idx) = input.trim().parse::<usize>() {
            if idx < prompt_options.len() {
                break prompt_options[idx].0.to_string();
            }
        }
        println!("Invalid selection. Try again.");
    };

    let mut config = load_config();
    config.prompt_style = style;
    save_config(&config);
    println!("\n\x1b[32m✔ Prompt style updated. Reloading shell...\x1b[0m");
    main();
}

struct XinuxHelper {
    commands: Vec<String>,
}

impl Completer for XinuxHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let input = &line[..pos];
        let input_parts: Vec<&str> = input.split_whitespace().collect();

        let mut suggestions = Vec::new();

        for command in &self.commands {
            let command_parts: Vec<&str> = command.split_whitespace().collect();

            if command_parts.len() < input_parts.len() {
                continue;
            }

            // Check exact match for all parts except the last
            let mut valid = true;
            for i in 0..input_parts.len() - 1 {
                if command_parts[i] != input_parts[i] {
                    valid = false;
                    break;
                }
            }

            // Check if the last part starts with the input's last part
            if valid {
                if let (Some(command_last_part), Some(input_last_part)) = (
                    command_parts.get(input_parts.len() - 1),
                    input_parts.last(),
                ) {
                    if command_last_part.starts_with(input_last_part) {
                        suggestions.push(Pair {
                            display: command.to_string(),
                            replacement: command.to_string(),
                        });
                    }
                }
            }
        }

        suggestions.sort_by(|a, b| a.display.cmp(&b.display));
        suggestions.dedup_by(|a, b| a.display == b.display);

        Ok((start, suggestions))
    }
}

impl Hinter for XinuxHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        let word_start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &line[word_start..pos];
        self.commands
            .iter()
            .find(|cmd| cmd.starts_with(word) && cmd != &word)
            .map(|cmd| cmd[word.len()..].to_string())
    }
}

impl Highlighter for XinuxHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let config = load_config();
        let mut highlighted = String::new();

        for word in line.split_whitespace() {
            if config.aliases.contains_key(word) || self.commands.contains(&word.to_string()) {
                // Highlight commands or aliases
                highlighted.push_str(&format!("\x1b[34m{}\x1b[0m ", word)); // Dark blue
            } else if word.starts_with('-') {
                // Highlight arguments
                highlighted.push_str(&format!("\x1b[33m{}\x1b[0m ", word)); // Yellow
            } else if PathBuf::from(word).is_dir() {
                // Highlight directories
                highlighted.push_str(&format!("\x1b[32m{}\x1b[0m ", word)); // Green
            } else if PathBuf::from(word).is_file() {
                // Highlight files
                highlighted.push_str(&format!("\x1b[36m{}\x1b[0m ", word)); // Cyan
            } else {
                // Highlight invalid input
                highlighted.push_str(&format!("\x1b[31m{}\x1b[0m ", word)); // Red
            }
        }

        Cow::Owned(highlighted.trim_end().to_string())
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[38;5;239m{}\x1b[0m", hint)) // Gray for hints
    }
}

impl Validator for XinuxHelper {
    fn validate(&self, _: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Helper for XinuxHelper {}

fn generate_prompt(style: &str, cwd_display: &str) -> String {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    match style {
        "two_line" => format!("\x1b[38;5;214m📁 {}\n🌌 > \x1b[0m", cwd_display),
        "boxy" => format!("\x1b[38;5;214m┌─[{}]\n└─> \x1b[0m", cwd_display),
        "minimal" => "\x1b[38;5;214m> \x1b[0m".to_string(),
        "classic" => format!("\x1b[38;5;172mλ {} > \x1b[0m", cwd_display),
        "bold_frame" => format!(
            "\x1b[38;5;214m╭─[{}@{}]\n╰─λ \x1b[0m",
            hostname, cwd_display
        ),
        "arrowed" => format!("\x1b[38;5;214m➜ {} \x1b[0m", cwd_display),
        "fancy_duo" => format!("\x1b[38;5;214m┃📁 {}\n┃🌌 > \x1b[0m", cwd_display),
        "thin_line" => format!("\x1b[38;5;214m───── {}\n➤ \x1b[0m", cwd_display),
        "modern_box" => format!("\x1b[38;5;214m╔═[{}]\n╚═➤ \x1b[0m", cwd_display),
        "bracketed" => format!("\x1b[38;5;214m[{}] λ ➤ \x1b[0m", cwd_display),
        "shell_style" => format!("\x1b[38;5;214m{}@🌌:{}$ \x1b[0m", hostname, cwd_display),

        "bold_double" => format!("\x1b[38;5;214m╔═╦═[{}]\n╚═╩═➤ \x1b[0m", cwd_display),
        "bold_rounded" => format!("\x1b[38;5;214m╭─╮ {}\n╰─╯ ➤ \x1b[0m", cwd_display),
        "bold_ascii" => format!("\x1b[38;5;214m[+++ {} +++]\n[     ➤ \x1b[0m", cwd_display),
        "bold_neon" => format!("\x1b[38;5;123m███ {}\n▀▀▀ ➤ \x1b[0m", cwd_display),
        "bold_circuit" => format!("\x1b[38;5;226m┌─⏁─[{}]\n└─⏚─➤ \x1b[0m", cwd_display),

        "cyberpunk" => format!("\x1b[38;5;129m⟦{}⟧\n⏁ \x1b[0m", cwd_display),
        "retro_green" => format!("\x1b[38;5;22m>----[{}]\n\\--> \x1b[0m", cwd_display),
        "space_age" => format!("\x1b[38;5;117m🚀 {}\n>---> \x1b[0m", cwd_display),
        "hacker" => format!("\x1b[38;5;46m[[{}]]\n# \x1b[0m", cwd_display),
        "pixel_art" => format!("\x1b[38;5;202m⡇{}⡇\n⠋ \x1b[0m", cwd_display),
        "matrix" => format!("\x1b[38;5;46m⟦{}⟧\n⏁ \x1b[0m", cwd_display),
        "steampunk" => format!("\x1b[38;5;94m⚙─[{}]\n\\/─➤ \x1b[0m", cwd_display),
        "ocean_wave" => format!("\x1b[38;5;38m~{}~\n» \x1b[0m", cwd_display),
        "fire" => format!("\x1b[38;5;202m{}*\n» \x1b[0m", cwd_display),
        "neon_sign" => format!("\x1b[38;5;123m{}*\n» \x1b[0m", cwd_display),
        "robot" => format!("\x1b[38;5;240m[{}]\n🤖 \x1b[0m", cwd_display),
        "terminal" => format!("\x1b[38;5;10m{}$\n> \x1b[0m", cwd_display),

        _ => format!("\x1b[38;5;214m🌌 [{}] > \x1b[0m", cwd_display),
    }
}

fn collect_autocomplete_commands(history_path: &PathBuf) -> Vec<String> {
    let mut commands = HashSet::new();

    if let Ok(history) = fs::read_to_string(history_path) {
        for line in history.lines() {
            let cmd = line.trim();
            if !cmd.is_empty() {
                commands.insert(cmd.to_string());
            }
        }
    }

    let builtins = vec![
        "cd",
        "ls",
        "clear",
        "exit",
        "quit",
        "help",
        "echo",
        "alias",
        "xinux config where",
        "xinux config prompt",
    ];
    for cmd in builtins {
        commands.insert(cmd.to_string());
    }

    if let Ok(path_var) = env::var("PATH") {
        for path in path_var.split(':') {
            let path = PathBuf::from(path);
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && is_executable(&path) {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            commands.insert(name.to_string());
                        }
                    }
                }
            }
        }
    }

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                commands.insert(name.to_string());
            }
        }
    }

    let config = load_config();
    commands.extend(config.aliases.keys().cloned());

    let mut sorted: Vec<String> = commands.into_iter().collect();
    sorted.sort();
    sorted
}

fn is_executable(path: &PathBuf) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

fn load_config() -> XinuxConfig {
    load_or_create_config()
}

fn save_config(config: &XinuxConfig) {
    let config_path = config_file_path();
    fs::write(&config_path, toml::to_string(config).unwrap()).expect("Failed to save config");
}

fn handle_alias(input: &str) {
    let parts: Vec<&str> = input[6..].splitn(2, '=').collect();
    if parts.len() != 2 {
        eprintln!("Usage: alias name=command");
        return;
    }

    let name = parts[0].trim().to_string();
    let command = parts[1].trim().to_string();

    if name.is_empty() || command.is_empty() {
        eprintln!("Alias name and command cannot be empty");
        return;
    }

    let mut config = load_config();
    config.aliases.insert(name.clone(), command.clone());
    save_config(&config);
    println!("Alias '{}' set to: {}", name, command);
}

fn main() {
    // Ensure the configuration directory exists before doing anything else
    let _ = xinux_dir();

    let config = load_config();

    // Run autostart commands
    for command in &config.autostart_commands {
        execute(command);
    }

    let history_path = history_file_path();
    let autocomplete_cmds = collect_autocomplete_commands(&history_path);
    let helper = XinuxHelper {
        commands: autocomplete_cmds,
    };

    let rl_config = ConfigBuilder::new()
        .history_ignore_dups(true)
        .unwrap()
        .auto_add_history(true)
        .build();

    let mut rl: Editor<XinuxHelper, rustyline::history::DefaultHistory> =
        Editor::with_config(rl_config).expect("Failed to initialize editor.");
    rl.set_helper(Some(helper));
    let _ = rl.load_history(&history_path);

    loop {
        let cwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or("?".into());
        let home = env::var("HOME").unwrap_or_default();
        let cwd_display = cwd.replace(&home, "~");
        let prompt = generate_prompt(&config.prompt_style, &cwd_display);

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();

                if input.starts_with("alias ") {
                    handle_alias(input);
                    continue;
                }

                match input {
                    "exit" | "quit" => break,
                    "help" => {
                        println!("Available commands:");
                        println!("├── cd: Teleport to another folder dimension!");
                        println!("├── ls: Summon a list of files and directories in your current realm.");
                        println!("├── clear: Wipe the terminal clean like a wizard's spell.");
                        println!("├── exit: Escape the Xinux universe and return to reality.");
                        println!("├── help: Summon this magical scroll of wisdom.");
                        println!("├── echo: Make the terminal repeat your words like a loyal parrot.");
                        println!("├── xinsay: Let Xin speak with the charm of cowsay!");
                        println!("├── alias: Forge shortcuts to commands like a true hacker-smith.");
                        println!("├── clear history: Erase your command history like a secret agent.");
                        println!("├── xinux config prompt: Redesign your prompt with style and flair.");
                        println!("├── xinux config where: Reveal the sacred location of the config file.");
                        println!("└── xinux reset: Reset Xinux to its pristine state, like a phoenix reborn.");
                    }
                    "clear" => {
                        if clearscreen::clear().is_err() {
                            println!("Failed to clear screen.");
                        }
                    }
                    "xinux config prompt" => {
                        reselect_prompt_style();
                        break;
                    }
                    "xinux config where" => {
                        let config_path = config_file_path();
                        println!(
                            "The configuration file is located at: {}",
                            config_path.display()
                        );
                    }
                    "xinux reset" => {
                        let config_path = config_file_path();
                        if fs::remove_file(&config_path).is_ok() {
                            println!("\x1b[32m✔ Configuration reset. Restarting Xinux...\x1b[0m");
                            main(); // Restart the application
                            break;
                        } else {
                            eprintln!("\x1b[31m✘ Failed to reset configuration.\x1b[0m");
                        }
                    }
                    _ if input == "clear history" => {
                        if let Err(e) = fs::remove_file(&history_path) {
                            eprintln!("Failed to clear history: {}", e);
                        } else {
                            println!("✔ History cleared.");
                            let new_cmds = collect_autocomplete_commands(&history_path);
                            if let Some(h) = rl.helper_mut() {
                                h.commands = new_cmds;
                            }
                        }
                    }
                    _ => {
                        let path = PathBuf::from(input);
                        if path.is_dir() {
                            if env::set_current_dir(&path).is_ok() {
                                println!("Changed directory to {}", path.display());
                                continue;
                            } else {
                                eprintln!("Failed to change directory to {}", path.display());
                            }
                        }

                        let config = load_config();
                        if let Some(alias) = config.aliases.get(input) {
                            execute(alias);
                        } else {
                            execute(input);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                
            }
            Err(ReadlineError::Eof) => {
                println!("\nEOF");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(e) = rl.save_history(&history_path) {
        eprintln!("Warning: Could not save history: {}", e);
    }
}
