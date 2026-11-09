//! Explicit on/off state only. Shell integration is disabled in this fork.
//! Historical hook helpers are retained only in upstream regression tests.

use std::io;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
const MARKER: &str = "# tuipo — Grammarly-style spell-check for terminal TUIs";

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
struct ShellHook {
    rc_path: PathBuf,
    hook_line: String,
}

pub fn run_init(_args: &[String]) -> i32 {
    eprintln!("tuipo: shell integration is disabled; run tuipo -- <application> explicitly");
    1
}

/// `tuipo off`: switch tuipo off until `tuipo on`.
pub fn run_off() -> i32 {
    let Some(path) = crate::config::off_switch_path() else {
        eprintln!("tuipo off: neither $XDG_CONFIG_HOME nor $HOME is set");
        return 1;
    };
    match switch_off(&path) {
        Ok(true) => {
            println!(
                "tuipo is off: open tabs stop underlining within a second, and new tabs start without it.\n\
                 Run `tuipo on` to turn it back on."
            );
            0
        }
        Ok(false) => {
            println!("tuipo is already off. Run `tuipo on` to turn it back on.");
            0
        }
        Err(e) => {
            eprintln!("tuipo off: couldn't create {}: {e}", path.display());
            1
        }
    }
}

/// `tuipo on`: undo `tuipo off`.
pub fn run_on() -> i32 {
    let Some(path) = crate::config::off_switch_path() else {
        eprintln!("tuipo on: neither $XDG_CONFIG_HOME nor $HOME is set");
        return 1;
    };
    match switch_on(&path) {
        Ok(true) => {
            println!(
                "tuipo is on: open tabs pick it back up within a second, and new tabs start with it.\n\
                 (Tabs opened while it was off stay plain until you open new ones.)"
            );
            0
        }
        Ok(false) => {
            println!("tuipo is already on.");
            0
        }
        Err(e) => {
            eprintln!("tuipo on: couldn't remove {}: {e}", path.display());
            1
        }
    }
}

/// Create the off switch. `Ok(true)` if tuipo was on.
fn switch_off(path: &Path) -> io::Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(
        path,
        "tuipo is switched off while this file exists; `tuipo on` removes it.\n",
    )?;
    Ok(true)
}

/// Remove the off switch. `Ok(true)` if tuipo was off.
fn switch_on(path: &Path) -> io::Result<bool> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

/// Remove any prior tuipo hook lines and their MARKER comments from rc
/// content. Recognises both the current absolute-path hook and the older
/// `command -v tuipo` style.
#[cfg(test)]
fn strip_old_hook(content: &str) -> String {
    let mut out_lines: Vec<&str> = Vec::with_capacity(content.lines().count());
    let mut skip_next_if_marker = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == MARKER {
            // Drop the marker; also drop the following hook line.
            skip_next_if_marker = true;
            continue;
        }
        if skip_next_if_marker {
            skip_next_if_marker = false;
            // Drop the immediately-following hook line.
            if trimmed.contains("TUIPO_ACTIVE") && trimmed.contains("exec") {
                continue;
            }
            // Otherwise put it back — the marker was orphaned.
            out_lines.push(line);
            continue;
        }
        // Also catch tuipo hook lines that were added by older versions
        // without the marker (or via manual edits).
        if trimmed.starts_with("[[ -z \"$TUIPO_ACTIVE\" ]]") && trimmed.contains("tuipo") {
            continue;
        }
        out_lines.push(line);
    }
    // Trim trailing blank lines.
    while out_lines.last().map(|l| l.trim().is_empty()) == Some(true) {
        out_lines.pop();
    }
    out_lines.join("\n")
}

#[cfg(test)]
fn detect_shell(
    shell_env: &str,
    home: &std::path::Path,
    exe_path: &std::path::Path,
) -> Option<ShellHook> {
    if shell_env.ends_with("/zsh") || shell_env == "zsh" {
        Some(ShellHook {
            rc_path: home.join(".zshrc"),
            hook_line: zsh_bash_hook(exe_path),
        })
    } else if shell_env.ends_with("/bash") || shell_env == "bash" {
        // On macOS, interactive bash reads .bash_profile rather than .bashrc;
        // prefer it if it exists, fall back to .bashrc.
        let bash_profile = home.join(".bash_profile");
        let bashrc = home.join(".bashrc");
        let rc_path = if bash_profile.exists() {
            bash_profile
        } else {
            bashrc
        };
        Some(ShellHook {
            rc_path,
            hook_line: zsh_bash_hook(exe_path),
        })
    } else {
        None
    }
}

/// Embed the absolute path to *this* tuipo binary in the hook line.
/// Avoids depending on `tuipo` being in `$PATH` after install.
#[cfg(test)]
fn zsh_bash_hook(exe_path: &std::path::Path) -> String {
    let p = exe_path.display();
    format!(r#"[[ -z "$TUIPO_ACTIVE" ]] && [[ -x "{p}" ]] && exec "{p}" -- "$SHELL""#)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn fake_exe() -> std::path::PathBuf {
        std::path::PathBuf::from("/usr/local/bin/tuipo")
    }

    #[test]
    fn detects_zsh_from_path() {
        let h = detect_shell("/bin/zsh", Path::new("/home/u"), &fake_exe());
        assert!(h.is_some());
        let h = h.unwrap();
        assert!(h.rc_path.ends_with(".zshrc"));
        assert!(h.hook_line.contains("TUIPO_ACTIVE"));
        assert!(h.hook_line.contains("/usr/local/bin/tuipo"));
    }

    #[test]
    fn detects_bash_from_path() {
        let h = detect_shell("/bin/bash", Path::new("/tmp/nonexistent-home"), &fake_exe());
        assert!(h.is_some());
        let h = h.unwrap();
        // No .bash_profile in fake home → .bashrc
        assert!(h.rc_path.ends_with(".bashrc"));
    }

    #[test]
    fn unknown_shell_is_rejected() {
        let h = detect_shell("/bin/fish", Path::new("/home/u"), &fake_exe());
        assert!(h.is_none());
    }

    #[test]
    fn empty_shell_env_is_rejected() {
        let h = detect_shell("", Path::new("/home/u"), &fake_exe());
        assert!(h.is_none());
    }

    #[test]
    fn hook_uses_quoted_shell_var() {
        let hook = zsh_bash_hook(&fake_exe());
        assert!(hook.contains(r#""$SHELL""#));
    }

    #[test]
    fn hook_is_idempotent_via_guard() {
        let hook = zsh_bash_hook(&fake_exe());
        assert!(hook.contains(r#"-z "$TUIPO_ACTIVE""#));
    }

    #[test]
    fn hook_embeds_absolute_path() {
        // Critical: the hook should NOT rely on `command -v tuipo`. It
        // must use the absolute path so it works regardless of $PATH.
        let exe = std::path::PathBuf::from("/Users/me/Code/tuipo/target/release/tuipo");
        let hook = zsh_bash_hook(&exe);
        assert!(
            hook.contains("/Users/me/Code/tuipo/target/release/tuipo"),
            "hook should embed the binary's absolute path, got: {hook}",
        );
        assert!(
            !hook.contains("command -v"),
            "hook shouldn't depend on PATH-based lookup; got: {hook}",
        );
    }

    #[test]
    fn hook_includes_executable_check() {
        // If the binary is moved/deleted, the hook should silently skip
        // rather than try to exec a non-existent file.
        let hook = zsh_bash_hook(&fake_exe());
        assert!(hook.contains("-x"));
    }

    #[test]
    fn strip_removes_marker_and_hook_block() {
        let rc = "\
# my existing zshrc
alias ll='ls -l'

# tuipo — Grammarly-style spell-check for terminal TUIs
[[ -z \"$TUIPO_ACTIVE\" ]] && command -v tuipo >/dev/null && exec tuipo -- \"$SHELL\"

# more user config
export FOO=bar
";
        let cleaned = strip_old_hook(rc);
        assert!(!cleaned.contains("TUIPO_ACTIVE"));
        assert!(!cleaned.contains("spell-check for terminal"));
        assert!(cleaned.contains("alias ll"));
        assert!(cleaned.contains("export FOO=bar"));
    }

    #[test]
    fn strip_handles_orphan_hook_without_marker() {
        let rc = "\
alias ll='ls -l'
[[ -z \"$TUIPO_ACTIVE\" ]] && exec /old/path/tuipo -- \"$SHELL\"
export FOO=bar
";
        let cleaned = strip_old_hook(rc);
        assert!(!cleaned.contains("TUIPO_ACTIVE"));
        assert!(cleaned.contains("alias ll"));
        assert!(cleaned.contains("export FOO=bar"));
    }

    #[test]
    fn strip_leaves_unrelated_lines_alone() {
        let rc = "\
alias ll='ls -l'
export FOO=bar
";
        assert_eq!(strip_old_hook(rc), rc.trim_end());
    }

    #[test]
    fn off_and_on_flip_the_switch_file_and_are_idempotent() {
        let dir = std::env::temp_dir().join(format!("tuipo-switch-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        // The config dir may not exist yet; `off` creates it.
        let path = dir.join("tuipo").join("off");

        assert!(switch_off(&path).unwrap(), "first off should switch");
        assert!(path.exists());
        assert!(!switch_off(&path).unwrap(), "second off is a no-op");

        assert!(switch_on(&path).unwrap(), "first on should switch");
        assert!(!path.exists());
        assert!(!switch_on(&path).unwrap(), "second on is a no-op");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
