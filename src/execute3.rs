use colored::*;
use rand::seq::IndexedRandom;

use crate::{execute2::run_external_command, xinsays::BFQ};

fn xinsay() {
    let quote = BFQ.choose(&mut rand::rng()).unwrap_or(&"Keep going!");

    let border_len = quote.len() + 4;
    let top = format!("  /{}\\", "-".repeat(border_len)).cyan();
    let mid = format!(" <  {}  >", quote).yellow();
    let bot = format!("  \\{}/", "-".repeat(border_len)).cyan();

    let bubble = format!("{}\n{}\n{}", top, mid, bot);
    let turtle = r#"
    _____     ____
  /      \   | o o |
 |        |_/  ___/
|    (_)   |  /
 \_________/ /
  |_|_| |_|_|
"#
    .green();

    println!("\n{}\n{}", bubble, turtle);
}

pub fn execute3(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };

    let args: Vec<&str> = parts.collect();

    match cmd {
        "xinsay" => xinsay(),
        "time" => {
            if let Some(subcommand) = args.get(0) {
            let start = std::time::Instant::now();
            let sub_args: Vec<&str> = args.iter().skip(1).cloned().collect();
            run_external_command(subcommand, &sub_args);
            let duration = start.elapsed();

            let hours = duration.as_secs() / 3600;
            let minutes = (duration.as_secs() % 3600) / 60;
            let seconds = duration.as_secs() % 60;
            let millis = duration.subsec_millis();
            let micors = duration.subsec_micros();
            let mut time_parts = Vec::new();
            if hours > 0 {
                time_parts.push(format!("{}h", hours).bold().blue().to_string());
            }
            if minutes > 0 {
                time_parts.push(format!("{}m", minutes).bold().green().to_string());
            }
            if seconds > 0 {
                time_parts.push(format!("{}s", seconds).bold().yellow().to_string());
            }
            if millis > 0 {
                time_parts.push(format!("{}ms", millis).bold().cyan().to_string());
            }
            if micors > 0 {
                time_parts.push(format!("{}µs", micors).bold().magenta().to_string());
            }
            println!(
                "~~~~+++~~~~\n{} {}",
                "Execution time:".bold().bright_white(),
                time_parts.join(" ")
            );
            } else {
            println!(
                "{} {}",
                "Usage:".bold().bright_red(),
                "time <command> [args...]".bright_white()
            );
            }
        }
        _ => run_external_command(cmd, &args),
    }
}
