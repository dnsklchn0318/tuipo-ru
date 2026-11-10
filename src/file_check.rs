//! Local checking of one explicitly selected file. No terminal proxy.
use crate::{config, spell};
use clap::{Parser, Subcommand};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
const MAX_BYTES: u64 = 1 << 20;

#[derive(Parser)]
#[command(
    version,
    about = "Check an explicitly selected UTF-8 file locally",
    arg_required_else_help = true
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Report RU/EN spelling mistakes without changing the file.
    Check { file: PathBuf },
    #[command(hide = true)]
    Engine,
}

fn read_file(path: &Path) -> Result<String, &'static str> {
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(path)
        .map_err(|_| "cannot open selected regular file")?;
    let meta = file
        .metadata()
        .map_err(|_| "cannot inspect selected file")?;
    if !meta.is_file() || meta.len() > MAX_BYTES {
        return Err("expected a regular file of at most 1 MiB");
    }
    read_text(file)
}
fn read_text(reader: impl Read) -> Result<String, &'static str> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "cannot read selected text")?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err("selected text exceeds 1 MiB");
    }
    String::from_utf8(bytes).map_err(|_| "selected text is not UTF-8")
}
fn escaped(text: &str) -> String {
    text.chars().flat_map(|c| {
        if c.is_control() || matches!(c, '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}') {
            c.escape_unicode().collect::<Vec<_>>()
        } else { vec![c] }
    }).collect()
}
fn check_file(path: &Path) -> Result<i32, &'static str> {
    use std::process::{Command, Stdio};
    let text = read_file(path)?;
    let mut command = Command::new(std::env::current_exe().map_err(|_| "engine unavailable")?);
    command
        .arg("__file_engine")
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for key in ["HOME", "XDG_CONFIG_HOME"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    let mut child = command.spawn().map_err(|_| "cannot start local engine")?;
    let sent = child
        .stdin
        .take()
        .ok_or("engine input unavailable")?
        .write_all(text.as_bytes());
    let output = child
        .wait_with_output()
        .map_err(|_| "local engine failed")?;
    if sent.is_err() || !output.status.success() {
        return Err("local engine or configuration failed");
    }
    let issues: Vec<spell::SpellIssue> =
        serde_json::from_slice(&output.stdout).map_err(|_| "invalid engine result")?;
    let mut count = 0;
    for issue in issues {
        let Some(prefix) = text.get(..issue.byte_start) else {
            return Err("invalid engine position");
        };
        let line = prefix.bytes().filter(|b| *b == b'\n').count() + 1;
        let col = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;
        println!(
            "{line}:{col}: {} -> {}",
            escaped(&issue.word),
            issue
                .suggestions
                .iter()
                .map(|s| escaped(s))
                .collect::<Vec<_>>()
                .join(", ")
        );
        count += 1;
    }
    Ok(i32::from(count > 0))
}
fn engine() -> Result<i32, &'static str> {
    config::load_global();
    if config::get().languages.is_empty() || config::get().validate_spelling().is_err() {
        return Err("invalid spelling configuration");
    }
    let text = read_text(std::io::stdin().lock())?;
    let mut checker = spell::SpellChecker::new();
    let issues: Vec<_> = checker
        .check(&text)
        .into_iter()
        .filter(|i| spell::is_actionable_category(i.category, config::get().grammar))
        .collect();
    serde_json::to_writer(std::io::stdout().lock(), &issues).map_err(|_| "engine output failed")?;
    Ok(0)
}
pub fn run(args: &[String]) -> i32 {
    let parsed =
        match Args::try_parse_from(std::iter::once("tuipo".to_owned()).chain(args.iter().cloned()))
        {
            Ok(parsed) => parsed,
            Err(error) => {
                let _ = error.print();
                return 2;
            }
        };
    let result = match parsed.command {
        Commands::Check { file } => check_file(&file),
        Commands::Engine => engine(),
    };
    match result {
        Ok(code) => code,
        Err(message) => {
            eprintln!("tuipo: {message}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_utf8() {
        assert!(read_text(&b"\xff"[..]).is_err());
        assert!(read_text(&vec![b'a'; MAX_BYTES as usize + 1][..]).is_err());
        assert_eq!(read_text(&b""[..]).unwrap(), "");
    }
    #[test]
    fn escape_terminal_and_bidi() {
        let value = escaped("привет\x1b[2J\n\u{202e}");
        assert!(!value.contains('\x1b'));
        assert!(!value.contains('\n'));
        assert!(!value.contains('\u{202e}'));
        assert!(value.starts_with("привет"));
    }
}
