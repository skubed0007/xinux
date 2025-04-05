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
        _ => run_external_command(cmd, &args),
    }
}
