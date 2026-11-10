use std::{
    fs,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "tuipo-file-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(p.join("config")).unwrap();
        Self(p)
    }
    fn run(&self, args: &[&std::ffi::OsStr]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_tuipo"))
            .args(args)
            .env_clear()
            .env("XDG_CONFIG_HOME", self.0.join("config"))
            .env("TUIPO_DEBUG_LOG", self.0.join("forbidden.log"))
            .output()
            .unwrap()
    }
    fn check(&self, bytes: &[u8]) -> Output {
        let p = self.0.join("text.txt");
        fs::write(&p, bytes).unwrap();
        let out = self.run(&["check".as_ref(), "--".as_ref(), p.as_os_str()]);
        assert_eq!(fs::read(p).unwrap(), bytes);
        assert!(!self.0.join("forbidden.log").exists());
        out
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn bilingual_positions_and_unchanged_file() {
    let f = Fixture::new();
    let out = f.check("Привет world\nпревет world ".as_bytes());
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8(out.stdout)
            .unwrap()
            .contains("2:1: превет")
    );
    assert!(out.stderr.is_empty());
}
#[test]
fn valid_empty_and_invalid_inputs() {
    let f = Fixture::new();
    for text in ["", "Привет world работающие люди "] {
        assert_eq!(f.check(text.as_bytes()).status.code(), Some(0));
    }
    for bytes in [vec![255], vec![b'a'; (1 << 20) + 1]] {
        assert_eq!(f.check(&bytes).status.code(), Some(2));
    }
}
#[test]
fn rejects_symlink_and_fifo() {
    let f = Fixture::new();
    let target = f.0.join("target");
    fs::write(&target, "hello").unwrap();
    let link = f.0.join("link");
    std::os::unix::fs::symlink(target, &link).unwrap();
    assert_eq!(
        f.run(&["check".as_ref(), link.as_os_str()]).status.code(),
        Some(2)
    );
    let fifo = f.0.join("fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert_eq!(
        f.run(&["check".as_ref(), fifo.as_os_str()]).status.code(),
        Some(2)
    );
}
#[test]
fn config_errors_suppress_contents() {
    let f = Fixture::new();
    fs::create_dir_all(f.0.join("config/tuipo")).unwrap();
    fs::write(f.0.join("config/tuipo/config.toml"), "secret-canary = [").unwrap();
    let out = f.check(b"hello");
    assert_eq!(out.status.code(), Some(2));
    assert!(!String::from_utf8_lossy(&out.stderr).contains("secret-canary"));
}
#[test]
fn custom_exceptions() {
    let f = Fixture::new();
    fs::create_dir_all(f.0.join("config/tuipo")).unwrap();
    fs::write(f.0.join("config/tuipo/dict.txt"), "превет\n").unwrap();
    assert_eq!(f.check("превет ".as_bytes()).status.code(), Some(0));
}
