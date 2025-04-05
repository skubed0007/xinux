use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use colored::*;
use humantime;
use sysinfo::System;
use whoami::{self, fallible};

use crate::execute3::execute3;

/// Main dispatcher
pub fn execute2(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args: Vec<&str> = parts.collect();

    match cmd {
        "info" => display_system_info(),
        "touch" => handle_touch(&args),
        _ => execute3(input),
    }
}

/// Touch-like command implementation with file type info
fn handle_touch(args: &[&str]) {
    if args.is_empty() {
        eprintln!("{}", "touch: missing filename".red().bold());
        return;
    }

    let filename = args[0];
    let path = Path::new(filename);

    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
    {
        Ok(_) => {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            println!("ext = {ext}");
            let (desc, icon) = match ext.as_str() {
                // Programming & Scripts
                "rs" => ("Rust source file", "🦀"),
                "py" => ("Python script", "🐍"),
                "c" => ("C source file", "💻"),
                "cpp" | "cc" => ("C++ source file", "🔧"),
                "zig" => ("Zig source file", "⚡"),
                "go" => ("Go source file", "🐹"),
                "js" => ("JavaScript file", "🌐"),
                "ts" => ("TypeScript file", "🔷"),
                "java" => ("Java source file", "☕"),
                "kt" => ("Kotlin file", "🅺"),
                "cs" => ("C# source file", "🎯"),
                "swift" => ("Swift file", "🕊️"),
                "rb" => ("Ruby script", "💎"),
                "php" => ("PHP script", "🐘"),
                "lua" => ("Lua script", "🌙"),
                "sh" | "bash" => ("Shell script", "📜"),
                "bat" => ("Batch file", "📄"),
                "pl" => ("Perl script", "🧬"),
                "r" => ("R script", "📊"),
                "asm" | "s" => ("Assembly source", "🏗️"),
                "v" | "vh" | "sv" => ("Verilog/SystemVerilog", "📶"),

                // Markup & Docs
                "html" | "htm" => ("HTML file", "📰"),
                "xml" => ("XML file", "🧾"),
                "md" => ("Markdown", "📝"),
                "txt" => ("Text file", "📄"),
                "rst" => ("reStructuredText", "🔠"),
                "adoc" => ("AsciiDoc", "📘"),

                // Config & Data
                "json" => ("JSON data", "🗂️"),
                "toml" => ("TOML config", "⚙️"),
                "yaml" | "yml" => ("YAML config", "⚙️"),
                "ini" => ("INI config", "🔧"),
                "cfg" | "conf" => ("Configuration file", "🧩"),
                "env" => ("Environment config", "🌱"),

                // Archives
                "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => ("Archive file", "🗜️"),

                // Fonts
                "ttf" | "otf" | "woff" | "woff2" => ("Font file", "🔤"),

                // Audio
                "mp3" | "wav" | "flac" | "ogg" | "m4a" => ("Audio file", "🎵"),

                // Video
                "mp4" | "mkv" | "webm" | "avi" | "mov" => ("Video file", "🎞️"),

                // Images
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "ico" => {
                    ("Image file", "🖼️")
                }

                // Disk images
                "iso" | "img" => ("Disk image", "💽"),

                // Database
                "db" | "sqlite" | "sql" | "mdb" | "accdb" => ("Database file", "🗄️"),

                // Code Projects & Build
                "makefile" | "mk" => ("Makefile", "🧱"),
                "gradle" | "pom" | "build" => ("Build script", "🏗️"),
                "lock" => ("Lock file", "🔒"),

                // Executables
                "exe" | "msi" | "bin" | "out" | "run" => ("Executable", "🚀"),
                "dll" | "so" | "dylib" => ("Dynamic library", "📚"),

                // Web & Misc
                "wasm" => ("WebAssembly", "🌐"),
                "tsv" | "csv" => ("Spreadsheet", "📈"),
                "log" => ("Log file", "🧾"),

                // Scripts & automation
                "ps1" => ("PowerShell script", "🖥️"),
                "dockerfile" => ("Dockerfile", "🐳"),

                // Backup & recovery
                "bak" => ("Backup file", "💾"),
                "old" => ("Old file version", "📁"),

                // Source packages
                "pkg" | "deb" | "rpm" | "apk" | "appimage" => ("Package file", "📦"),

                // Emulation/ROMs
                "cue" | "rom" => ("ROM file", "🕹️"),

                // Default fallback
                "" => ("Unnamed file", "📄"),
                _ => ("Unknown file type", "📦"),
            };

            println!(
                "{} {} {} {} ({})",
                "✔️ ".green().bold(),
                "Created".bright_green(),
                filename.bright_white().underline(),
                "➜".dimmed(),
                format!("{} {}", desc, icon).blue()
            );
        }
        Err(e) => {
            eprintln!(
                "{} {}: {}",
                "❌".red().bold(),
                "Failed to create file".bright_red(),
                e.to_string().dimmed()
            );
        }
    }
}

/// Executes system commands as fallback
pub(crate) fn run_external_command(cmd: &str, args: &[&str]) {
    let status_result = Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status_result {
        Ok(status) => {
            if !status.success() {
                if let Some(code) = status.code() {
                    eprintln!(
                        "{} {}",
                        "[ERROR]".bright_red().bold(),
                        format!(
                            "The command `{}` exited with a non-zero status code: {}.",
                            cmd.bright_yellow(),
                            code.to_string().bright_red()
                        )
                    );
                } else {
                    eprintln!(
                        "{} {}",
                        "[ERROR]".bright_red().bold(),
                        format!(
                            "The command `{}` was terminated by a signal and did not exit normally.",
                            cmd.bright_yellow()
                        )
                    );
                }
            }
        }
        Err(error) => {
            use std::io::ErrorKind;
            let detailed_msg = match error.kind() {
                ErrorKind::NotFound => format!(
                    "The command `{}` could not be found. \
                    Please ensure it is installed and available in your PATH.",
                    cmd.bright_yellow()
                ),
                ErrorKind::PermissionDenied => format!(
                    "Permission denied while attempting to execute `{}`. \
                    Make sure the file is executable and you have the required permissions.",
                    cmd.bright_yellow()
                ),
                _ => format!(
                    "Failed to execute `{}` due to an unexpected error: {}",
                    cmd.bright_yellow(),
                    error.to_string().bright_red()
                ),
            };

            eprintln!(
                "{} {}",
                "[ERROR]".bright_red().bold(),
                detailed_msg
            );
        }
    }
}

/// Fancy system info display (like neofetch)
fn display_system_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let host = format!(
        "{}@{}",
        whoami::username(),
        fallible::hostname().unwrap_or_else(|_| "Unknown".into())
    );
    let kernel = sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".into());
    
    let distro = sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown".into());
    let uptime = humantime::format_duration(Duration::from_secs(sysinfo::System::uptime()));
    let uptime_str = uptime.to_string();

    let cpu_info = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or("Unknown CPU".into());
    let cpu_count = sys.cpus().len();
    let cpu_str = format!("{} ({})", cpu_info, cpu_count);

    let mem_used = sys.used_memory() / 1024;
    let mem_total = sys.total_memory() / 1024;
    let mem_str = format!(
        "{:.1} GB / {:.1} GB",
        mem_used as f32 / 1024.0,
        mem_total as f32 / 1024.0
    );

    let load = format!("{:.0}%", sysinfo::System::load_average().one * 25.0); // Rough estimate

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "fish".into());
    let term = std::env::var("TERM").unwrap_or_else(|_| "gnome-terminal".into());

    // Replace with real detection if needed
    let resolution = "1920x1080".to_string();
    let machine = "INTEL H81".to_string();
    let de = "X-Cinnamon".to_string();
    let wm = "Mutter (Muffin) (X11)".to_string();
    let packages = get_package_counts();

    let entries = vec![
        ("Host", host),
        ("Machine", machine),
        ("Kernel", kernel),
        ("Distro", distro),
        ("DE", de),
        ("WM", wm),
        ("Packages", packages),
        ("Shell", shell),
        ("Terminal", term),
        ("Resolution", resolution),
        ("Uptime", uptime_str),
        ("CPU", cpu_str),
        ("CPU Load", load),
        ("Memory", mem_str),
    ];

    let max_key_len = entries.iter().map(|(k, _)| k.len()).max().unwrap_or(0);

    println!("\n{}", "System Info".bold().bright_cyan());

    for (i, (key, val)) in entries.iter().enumerate() {
        let branch = if i == entries.len() - 1 {
            "└──"
        } else {
            "├──"
        };
        let pad_key = format!("{:width$}", key, width = max_key_len);
        println!(
            "{} {} {} {}",
            branch.bright_black(),
            pad_key.bright_yellow(),
            "➜".dimmed(),
            val.bright_white()
        );
    }

    println!();
}

fn get_package_counts() -> String {
    let managers = vec![
        ("pacman", "pacman -Q | wc -l"),
        ("apt", "dpkg -l | grep '^ii' | wc -l"),
        ("dnf", "dnf list installed | wc -l"),
        ("rpm", "rpm -qa | wc -l"),
        ("flatpak", "flatpak list | wc -l"),
        ("snap", "snap list | wc -l"),
        ("eopkg", "eopkg list-installed | wc -l"),
        ("nix", "nix-env -q | wc -l"),
        (
            "cargo",
            "cargo install --list | grep -cE '^[a-zA-Z0-9_-]+ v'",
        ),
        ("brew", "brew list | wc -l"),
        ("pip", "pip list | wc -l"),
        ("conda", "conda list | wc -l"),
        ("npm", "npm list -g --depth=0 | grep -cE '^[├└]──'"),
        ("winget", "winget list | find /c /v \"\""),
        ("scoop", "scoop list | find /c /v \"\""),
        ("choco", "choco list -l | find /c /v \"\""),
    ];

    let mut results = vec![];

    for (name, cmd) in managers {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd]).output()
        } else {
            Command::new("bash").arg("-c").arg(cmd).output()
        };

        if let Ok(out) = output {
            if let Ok(txt) = String::from_utf8(out.stdout) {
                let count = txt.trim();
                // only show if output is numeric and non-zero
                if !count.is_empty() && count.chars().all(|c| c.is_digit(10)) && count != "0" {
                    results.push(format!("{}: {}", name, count));
                }
            }
        }
    }

    if results.is_empty() {
        "No package managers found or none returned data".into()
    } else {
        results.join(", ")
    }
}
