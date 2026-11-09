//! Opt-in, metadata-only logging. No input, output, words, suggestions,
//! diagnostics from children, paths, environment values or panic payloads.
//! TUIPO_DEBUG_LOG selects a private regular file; content logging has no switch.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use crate::echo::EchoMatcher;
use crate::screen::ScreenState;

/// Shared file handle + serialization lock. The same file may be opened by
/// both the stdin and output threads, but each line goes through this Mutex
/// so it lands atomically rather than getting shredded with another thread's
/// output. `OnceLock` ensures we only open the file once per process even
/// when `DebugLog::from_env` is called from multiple threads.
static SHARED: OnceLock<Option<Arc<Mutex<std::fs::File>>>> = OnceLock::new();

pub struct DebugLog {
    file: Option<Arc<Mutex<std::fs::File>>>,
    start: Instant,
}

impl DebugLog {
    pub fn from_env() -> Self {
        let shared = SHARED
            .get_or_init(|| {
                std::env::var_os("TUIPO_DEBUG_LOG").and_then(|p| {
                    open_private_log(std::path::Path::new(&p))
                        .ok()
                        .map(|f| Arc::new(Mutex::new(f)))
                })
            })
            .clone();
        let log = Self {
            file: shared,
            start: Instant::now(),
        };
        // Two threads call `from_env` (stdin + output), so two BOOT lines
        // appear per session. Grep them out if it bothers you.
        log.line("BOOT", "tuipo session started");
        log
    }

    pub fn enabled(&self) -> bool {
        self.file.is_some()
    }

    pub fn log_input(&self) {
        self.line("INPUT", "updated");
    }

    pub fn log_lints(&self, count: usize) {
        self.line("LINTS", &format!("issues={count}"));
    }

    pub fn log_engine(&self, event: EngineMetadata) {
        let payload = match event {
            EngineMetadata::InProcess => "in-process mode".into(),
            EngineMetadata::Spawn { pid } => format!("spawn pid={pid}"),
            EngineMetadata::Ready { pid, elapsed_ms } => {
                format!("ready pid={pid} in {elapsed_ms}ms")
            }
            EngineMetadata::Stop {
                reason,
                pid,
                served,
            } => format!("stop reason={reason} pid={pid} served={served}"),
            EngineMetadata::Failure { reason, pid, count } => {
                format!("{reason} pid={pid} failure={count}")
            }
            EngineMetadata::Fallback => "falling back to in-process linting".into(),
        };
        self.line("ENGINE", &payload);
    }

    /// `tuipo off` / `tuipo on` noticed by a running session.
    pub fn log_switch(&self, on: bool) {
        self.line("SWITCH", if on { "on" } else { "off" });
    }

    pub fn log_boundary(&self) {
        self.line("BOUNDARY", "input reset");
    }

    pub fn log_prints(&self, count: usize) {
        self.line("PRINTS", &format!("count={count}"));
    }

    pub fn log_paint(&self, lint_count: usize) {
        if !self.enabled() {
            return;
        }
        self.line("PAINT", &format!("painted {lint_count} issue(s)"));
    }

    pub fn log_resize(&self, cols: u16, rows: u16) {
        if !self.enabled() {
            return;
        }
        self.line("RESIZE", &format!("winch -> ({cols}x{rows})"));
    }

    pub fn log_fix(&self) {
        self.line("FIX", "applied");
    }

    pub fn log_matcher(&self, matcher: &EchoMatcher) {
        if !self.enabled() {
            return;
        }
        let payload = format!(
            "positions={} pending={} lints={}",
            matcher.known_positions(),
            matcher.pending_len(),
            matcher.lints().len(),
        );
        self.line("MATCHER", &payload);
    }

    pub fn log_screen(&self, state: &ScreenState) {
        if !self.enabled() {
            return;
        }
        let payload = format!(
            "cursor=({},{}) cols={} rows={} alt={} wrap={}",
            state.cursor.row,
            state.cursor.col,
            state.cols,
            state.rows,
            state.alt_screen,
            state.pending_wrap,
        );
        self.line("SCREEN", &payload);
    }

    fn line(&self, kind: &str, payload: &str) {
        let Some(file) = self.file.as_ref() else {
            return;
        };
        let elapsed = self.start.elapsed().as_millis();
        // Format the whole line first so the single write_all is atomic
        // enough; mutex serializes the line across threads.
        let line = format!("T+{elapsed:08} {kind:<8} {payload}\n");
        if let Ok(mut f) = file.lock() {
            let _ = f.write_all(line.as_bytes());
            let _ = f.flush();
        }
    }
}

/// Only fixed lifecycle labels and numeric values may cross this API.
pub enum EngineMetadata {
    InProcess,
    Spawn {
        pid: u32,
    },
    Ready {
        pid: u32,
        elapsed_ms: u128,
    },
    Stop {
        reason: &'static str,
        pid: u32,
        served: u64,
    },
    Failure {
        reason: &'static str,
        pid: u32,
        count: u32,
    },
    Fallback,
}

fn open_private_log(path: &std::path::Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK | nix::libc::O_CLOEXEC)
        .open(path)?;
    let meta = file.metadata()?;
    if !meta.is_file() || meta.mode() & 0o077 != 0 || meta.nlink() != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "debug log must be a private, single-link regular file",
        ));
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::MetadataExt;

    #[test]
    fn metadata_log_never_serializes_input_output_or_issues() {
        let dir = std::env::temp_dir().join(format!("tuipo-log-policy-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("metadata.log");
        let _ = std::fs::remove_file(&path);
        let log = DebugLog {
            file: Some(Arc::new(Mutex::new(open_private_log(&path).unwrap()))),
            start: Instant::now(),
        };
        let mut state = crate::input::InputState::with_checker();
        state.feed_bytes(b"promptCanary_9d02 passwordCanary_43 tokenCanary_abcd");
        log.log_input();
        log.log_lints(state.issues().len());
        log.log_prints("outputCanary_e23".chars().count());
        log.log_fix();
        log.log_boundary();
        log.log_engine(EngineMetadata::Fallback);
        log.log_engine(EngineMetadata::Spawn { pid: 42 });
        log.log_engine(EngineMetadata::Ready {
            pid: 42,
            elapsed_ms: 10,
        });
        log.log_engine(EngineMetadata::Stop {
            reason: "idle",
            pid: 42,
            served: 1,
        });
        log.log_matcher(&EchoMatcher::new());
        log.log_screen(crate::screen::ScreenObserver::new(80, 24).state());
        log.log_resize(80, 24);
        log.log_paint(1);
        log.log_switch(false);
        let text = std::fs::read_to_string(&path).unwrap();
        for canary in [
            "promptCanary",
            "passwordCanary",
            "tokenCanary",
            "outputCanary",
            "buf=",
            "→",
        ] {
            assert!(!text.contains(canary), "content escaped into metadata log");
        }
        assert!(text.contains("LINTS") && text.contains("FIX") && text.contains("ENGINE"));
        assert_eq!(path.metadata().unwrap().mode() & 0o777, 0o600);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn rejects_symlinks_hardlinks_and_public_files() {
        use std::os::unix::fs::{PermissionsExt, symlink};
        let dir = std::env::temp_dir().join(format!("tuipo-log-files-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("target");
        std::fs::write(&path, "unchanged").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(open_private_log(&path).is_err());
        let link = dir.join("link");
        symlink(&path, &link).unwrap();
        assert!(open_private_log(&link).is_err());
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        std::fs::hard_link(&path, dir.join("hardlink")).unwrap();
        assert!(open_private_log(&path).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "unchanged");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
