//! Persistent configuration loaded from `~/.config/tuipo/config.toml`
//! (or `$XDG_CONFIG_HOME/tuipo/config.toml`). All settings are optional;
//! missing keys fall back to the documented defaults.
//!
//! ## Why a file, not just env vars
//!
//! Env vars (`TUIPO_PAINT_OFF`, `TUIPO_TAB_FIX`, etc.) work fine for
//! one-off toggling but they're invisible: the user has to remember they
//! set them, the docs are scattered across `main.rs`'s help text, and
//! there's no central place to discover what knobs exist. The TOML config
//! is the discoverable, persistent layer; env vars stay supported as
//! per-session overrides that always win over the file. Default values
//! match the historical "minimal, surprising nothing" behavior — picker
//! is off, Tab passes through, status row is hidden — so a user who
//! never writes a config file behaves exactly as before.
//!
//! ## Loading
//!
//! [`Config::load_global`] is called once from `main` and stores the
//! result in a process-global `OnceLock`. All reads go through
//! [`get`] which falls back to `Config::default()` if `load_global` was
//! never called (handy in tests). The file is parsed once at startup —
//! live reload (filesystem watcher) is a follow-up.
//!
//! ## Env override convention
//!
//! For every config field that has an equivalent env var, the env var
//! wins. This means existing users with shell init that sets
//! `TUIPO_TAB_FIX=1` keep working; the file is just a more discoverable
//! way to express the same choice.

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use serde::Deserialize;

use crate::paint::{self, UnderlineStyle};

/// Default pause window before painting. Match the previous `PAUSE_MS`
/// constant — config can override per-user but the default is unchanged.
pub const DEFAULT_PAUSE_MS: u64 = 150;

/// Default idle delay before the passive hover tooltip appears when the
/// picker feature is enabled. Long enough to feel intentional (not popping
/// up from accidental cursor passes) but short enough to feel responsive.
pub const DEFAULT_HOVER_MS: u64 = 250;

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UnderlineMode {
    /// Detect at runtime: PLAIN on Apple Terminal (mis-parses colon-form
    /// SGR sub-parameters), FANCY (curly red) elsewhere.
    #[default]
    Auto,
    /// Always plain `\x1b[4m` / `\x1b[24m`.
    Plain,
    /// Always curly red `\x1b[4:3m\x1b[58:2::255:0:0m`. Useful in tmux
    /// which strips `TERM_PROGRAM` and would otherwise default to PLAIN.
    Fancy,
    /// Quiet high-contrast word highlight, including Terminal.app.
    Highlight,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub languages: Vec<String>,
    pub interactive: bool,
    pub highlight_only: bool,
    pub next_key: String,
    pub previous_key: String,
    pub russian_aff_path: Option<String>,
    pub russian_dic_path: Option<String>,
    /// Master switch for the inline paint overlay. Off → tuipo is pure
    /// passthrough.
    pub paint: bool,
    /// Which SGR style the painter emits. See [`UnderlineMode`].
    pub underline: UnderlineMode,
    /// `true` → Tab triggers spell-fix behavior (apply top suggestion, or
    /// open the picker if `picker` is also enabled). Default-off so the
    /// wrapped child's normal Tab semantics (shell completion, vim
    /// indent, slash-command pickers) are unaffected.
    pub tab_fix: bool,
    /// `true` → enable the hover/Tab suggestion picker UI. Implies the
    /// Tab-fix path runs through the picker rather than auto-applying.
    /// Default-off; the picker overlay can fight hosts that aggressively
    /// redraw their bottom area, so it's an explicit opt-in.
    pub picker: bool,
    /// `true` → show the idle issue-count status row at the bottom of
    /// the wrapped terminal. Default-off because hosts that redraw their
    /// own footer fight us for that row.
    pub status_row: bool,
    /// `true` → surface a narrow whitelist of high-precision grammar
    /// lints (subject-verb agreement, malapropisms, eggcorns, usage)
    /// alongside spelling. Default-off because broader grammar checking
    /// misfires on imperative terminal prompts (missing periods,
    /// lowercase starts, fragments). Only the whitelist is enabled
    /// here; broader categories stay rejected at the spell engine
    /// layer — see [`crate::spell::IssueCategory::from_kind`].
    pub grammar: bool,
    /// Idle ms before the painter fires the first paint after a
    /// keystroke. Default 150ms — matches the historical const.
    pub pause_ms: u64,
    /// Idle ms before the picker hover tooltip appears once the buffer
    /// cursor is inside a flagged span. Default 250ms.
    pub hover_ms: u64,
    /// Optional override for the user dict file path. When unset,
    /// `~/.config/tuipo/dict.txt` (or `$XDG_CONFIG_HOME/tuipo/dict.txt`)
    /// is used.
    pub dict_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            languages: vec!["ru_RU".into(), "en_US".into()],
            interactive: true,
            highlight_only: false,
            next_key: "ctrl+n".into(),
            previous_key: "ctrl+p".into(),
            russian_aff_path: None,
            russian_dic_path: None,
            paint: true,
            underline: UnderlineMode::Auto,
            tab_fix: false,
            picker: false,
            status_row: false,
            grammar: false,
            pause_ms: DEFAULT_PAUSE_MS,
            hover_ms: DEFAULT_HOVER_MS,
            dict_path: None,
        }
    }
}

impl Config {
    /// Load from the user's config path. Missing file → defaults. Parse
    /// or read error → disabled input processing + a content-free warning.
    /// A missing file alone uses the documented defaults.
    pub fn load() -> Self {
        let Some(path) = default_config_path() else {
            return Self::default();
        };
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Self::default(),
            Err(_) => {
                eprintln!("tuipo: configuration unavailable; input processing disabled");
                return Self::disabled();
            }
        };
        match toml::from_str::<Config>(&text) {
            Ok(cfg) => cfg,
            Err(_) => {
                eprintln!(
                    "tuipo: invalid configuration; input processing disabled (contents suppressed)"
                );
                Self::disabled()
            }
        }
    }

    fn disabled() -> Self {
        Self {
            languages: Vec::new(),
            paint: false,
            interactive: false,
            highlight_only: true,
            ..Self::default()
        }
    }

    /// Whether the painter should run at all. Env override:
    /// `TUIPO_PAINT_OFF=<anything>` forces off.
    pub fn paint_enabled(&self) -> bool {
        if std::env::var_os("TUIPO_PAINT_OFF").is_some() {
            return false;
        }
        self.paint
    }

    /// Whether Tab is intercepted for spell-fix behavior. Env override:
    /// `TUIPO_TAB_FIX=1` forces on. The picker switch implies tab_fix
    /// because the picker is only reachable via Tab.
    pub fn tab_fix_enabled(&self) -> bool {
        if self.highlight_only {
            return false;
        }
        if matches!(std::env::var("TUIPO_TAB_FIX").as_deref(), Ok("1")) {
            return true;
        }
        !self.highlight_only && (self.tab_fix || self.picker_enabled_raw())
    }

    /// Whether the picker UI is enabled. No env override for this one
    /// yet — config-only, since the picker is more invasive than the
    /// other flags and adding env soup didn't seem worth it.
    #[allow(dead_code)] // wired up in the picker phase (tier 2)
    pub fn picker_enabled(&self) -> bool {
        self.picker_enabled_raw()
    }

    fn picker_enabled_raw(&self) -> bool {
        self.picker && !self.highlight_only
    }

    /// Whether the bottom-row status overlay is drawn. Env override:
    /// `TUIPO_STATUS=on`.
    pub fn status_row_enabled(&self) -> bool {
        if matches!(std::env::var("TUIPO_STATUS").as_deref(), Ok("on")) {
            return true;
        }
        self.status_row
    }

    /// Whether the narrow grammar lint whitelist is surfaced (painted +
    /// pickable). Env override: `TUIPO_GRAMMAR=1` forces on. Default-off
    /// — broader grammar checking has a high false-positive rate on
    /// terminal prompts (imperatives, no terminal punctuation, etc.), so
    /// the user must explicitly opt in.
    pub fn grammar_enabled(&self) -> bool {
        if matches!(std::env::var("TUIPO_GRAMMAR").as_deref(), Ok("1")) {
            return true;
        }
        self.grammar
    }

    /// Pause window before the painter fires its first post-keystroke
    /// paint. Currently config-only (no env override) because the
    /// historical default was a `const` — nobody was tuning this.
    #[allow(dead_code)] // wired up in a follow-up that replaces PAUSE_MS
    pub fn pause_duration(&self) -> Duration {
        Duration::from_millis(self.pause_ms)
    }

    /// Idle window before the picker hover tooltip appears.
    #[allow(dead_code)] // wired up in the picker phase (tier 2)
    pub fn hover_duration(&self) -> Duration {
        Duration::from_millis(self.hover_ms)
    }

    /// Resolve the underline SGR style. Env overrides:
    /// `TUIPO_PLAIN_UNDERLINE=<anything>` → PLAIN.
    /// `TUIPO_FANCY_UNDERLINE=<anything>` → FANCY.
    /// Otherwise the config's [`UnderlineMode`] wins; `Auto` falls back
    /// to `TERM_PROGRAM` detection (Apple_Terminal → PLAIN, else FANCY).
    pub fn underline_style(&self) -> UnderlineStyle {
        if std::env::var_os("TUIPO_PLAIN_UNDERLINE").is_some() {
            return paint::PLAIN;
        }
        if std::env::var_os("TUIPO_FANCY_UNDERLINE").is_some() {
            return paint::FANCY;
        }
        match self.underline {
            UnderlineMode::Plain => paint::PLAIN,
            UnderlineMode::Fancy => paint::FANCY,
            UnderlineMode::Highlight => paint::HIGHLIGHT,
            UnderlineMode::Auto => match std::env::var("TERM_PROGRAM").as_deref() {
                Ok("Apple_Terminal") => paint::PLAIN,
                _ => paint::FANCY,
            },
        }
    }
}

pub(crate) static GLOBAL: OnceLock<Config> = OnceLock::new();

/// Initialize the process-global config. Should be called once from
/// `main` before any thread reads from it. Subsequent calls are no-ops —
/// tests run with the default config because they never call this.
pub fn load_global() {
    let _ = GLOBAL.set(Config::load());
}

/// Read the process-global config. If `load_global` was never called
/// (tests, library use) returns a reference to a default config.
pub fn get() -> &'static Config {
    GLOBAL.get_or_init(Config::default)
}

/// Where `Config::load` reads from — `$XDG_CONFIG_HOME/tuipo/config.toml`
/// if set, else `~/.config/tuipo/config.toml`. Returns None only when
/// neither `XDG_CONFIG_HOME` nor `HOME` is set, which is rare enough to
/// be a "give up gracefully" case.
/// tuipo's config directory: `$XDG_CONFIG_HOME/tuipo`, else
/// `~/.config/tuipo`. Holds `config.toml`, `dict.txt` and the off switch.
pub fn config_dir() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg).join("tuipo"));
    }
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".config").join("tuipo"))
}

pub fn default_config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.toml"))
}

/// While this file exists tuipo is switched off (`tuipo off` creates it,
/// `tuipo on` removes it): `tuipo -- <cmd>` just runs the command, and
/// running sessions stop checking and painting within a second.
pub fn off_switch_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("off"))
}

/// Whether `tuipo off` is in effect.
pub fn switched_off() -> bool {
    match off_switch_path() {
        Some(path) => match std::fs::symlink_metadata(path) {
            Ok(_) => true,
            Err(err) => err.kind() != std::io::ErrorKind::NotFound,
        },
        None => true,
    }
}

/// Only inspect the presence of relevant flags, never their values.
pub fn restricted_launch() -> bool {
    [
        "TUIPO_PRIVATE",
        "TUIPO_ACTIVE",
        "SSH_CONNECTION",
        "SSH_CLIENT",
        "SSH_TTY",
    ]
    .iter()
    .any(|key| std::env::var_os(key).is_some())
}

impl Config {
    /// Serialize to a TOML string suitable for writing back to
    /// `~/.config/tuipo/config.toml`. Hand-rolled rather than via
    /// `toml::to_string` so we don't pull in the `display` feature of
    /// the `toml` crate — keeps the dependency tree lean. Output is
    /// stable line-by-line so the TUI can produce diff-friendly writes.
    pub fn to_toml_string(&self) -> String {
        let mut s = String::new();
        s.push_str("# Generated by `tuipo config`. Hand-edits are preserved on next\n");
        s.push_str("# save, but unknown keys will fail to parse (see\n");
        s.push_str("# `deny_unknown_fields` in src/config.rs).\n\n");
        s.push_str(&format!("paint = {}\n", self.paint));
        s.push_str(&format!(
            "underline = \"{}\"\n",
            underline_mode_str(self.underline)
        ));
        s.push_str(&format!("tab_fix = {}\n", self.tab_fix));
        s.push_str(&format!("picker = {}\n", self.picker));
        s.push_str(&format!("status_row = {}\n", self.status_row));
        s.push_str(&format!("grammar = {}\n", self.grammar));
        s.push_str(&format!("pause_ms = {}\n", self.pause_ms));
        s.push_str(&format!("hover_ms = {}\n", self.hover_ms));
        if let Some(p) = &self.dict_path {
            // TOML strings: escape backslash and double quote. We don't
            // expect other special chars in a path; keeping the escape
            // table small means no surprise interactions with multibyte.
            let escaped = p.replace('\\', "\\\\").replace('"', "\\\"");
            s.push_str(&format!("dict_path = \"{escaped}\"\n"));
        }
        s.push_str(&format!("languages = {:?}\ninteractive = {}\nhighlight_only = {}\nnext_key = {:?}\nprevious_key = {:?}\n", self.languages, self.interactive, self.highlight_only, self.next_key, self.previous_key));
        for (key, value) in [
            ("russian_aff_path", &self.russian_aff_path),
            ("russian_dic_path", &self.russian_dic_path),
        ] {
            if let Some(value) = value {
                s.push_str(&format!("{key} = {value:?}\n"));
            }
        }
        s
    }

    pub fn completion_direction(&self, byte: u8) -> Option<isize> {
        if !self.interactive || self.highlight_only {
            return None;
        }
        let matches = |key: &str| {
            key.strip_prefix("ctrl+")
                .filter(|k| k.len() == 1)
                .is_some_and(|k| {
                    let c = k.as_bytes()[0];
                    c.is_ascii_alphabetic() && byte == (c.to_ascii_lowercase() - b'a' + 1)
                })
        };
        if matches(&self.next_key) {
            Some(1)
        } else if matches(&self.previous_key) {
            Some(-1)
        } else {
            None
        }
    }

    pub fn validate_spelling(&self) -> anyhow::Result<()> {
        for language in &self.languages {
            anyhow::ensure!(
                matches!(language.as_str(), "ru_RU" | "en_US"),
                "unsupported language: {language}"
            );
        }
        anyhow::ensure!(
            self.russian_aff_path.is_some() == self.russian_dic_path.is_some(),
            "supply both russian_aff_path and russian_dic_path"
        );
        if let (Some(aff), Some(dic)) = (&self.russian_aff_path, &self.russian_dic_path) {
            let aff_text = std::fs::read_to_string(aff)?;
            let dic_text = std::fs::read_to_string(dic)?;
            spellbook::Dictionary::new(&aff_text, &dic_text)
                .map_err(|err| anyhow::anyhow!("invalid Russian dictionary: {err:?}"))?;
        }
        Ok(())
    }

    pub fn completion_enabled(&self) -> bool {
        self.interactive && !self.highlight_only
    }
}

fn underline_mode_str(m: UnderlineMode) -> &'static str {
    match m {
        UnderlineMode::Auto => "auto",
        UnderlineMode::Plain => "plain",
        UnderlineMode::Fancy => "fancy",
        UnderlineMode::Highlight => "highlight",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_setting_round_trips() {
        let cfg = Config {
            underline: UnderlineMode::Highlight,
            ..Config::default()
        };
        let parsed: Config = toml::from_str(&cfg.to_toml_string()).unwrap();
        assert_eq!(parsed.underline, UnderlineMode::Highlight);
    }

    #[test]
    fn defaults_are_minimal() {
        // Backstop for "default tuipo behavior is unchanged" — any new
        // setting whose default is non-trivial should be a deliberate
        // choice flagged by this test failing.
        let c = Config::default();
        assert!(c.paint, "paint defaults to on");
        assert!(!c.tab_fix, "tab_fix defaults to off");
        assert!(!c.picker, "picker defaults to off");
        assert!(!c.status_row, "status_row defaults to off");
        assert!(!c.grammar, "grammar defaults to off");
        assert_eq!(c.pause_ms, DEFAULT_PAUSE_MS);
        assert_eq!(c.hover_ms, DEFAULT_HOVER_MS);
        assert!(c.dict_path.is_none());
        assert_eq!(c.underline, UnderlineMode::Auto);
    }

    #[test]
    fn parses_full_config() {
        let toml_src = r#"
            paint = false
            underline = "fancy"
            tab_fix = true
            picker = true
            status_row = true
            grammar = true
            pause_ms = 250
            hover_ms = 400
            dict_path = "/tmp/words.txt"
        "#;
        let c: Config = toml::from_str(toml_src).expect("parse");
        assert!(!c.paint);
        assert_eq!(c.underline, UnderlineMode::Fancy);
        assert!(c.tab_fix);
        assert!(c.picker);
        assert!(c.status_row);
        assert!(c.grammar);
        assert_eq!(c.pause_ms, 250);
        assert_eq!(c.hover_ms, 400);
        assert_eq!(c.dict_path.as_deref(), Some("/tmp/words.txt"));
    }

    #[test]
    fn partial_config_uses_defaults_for_missing() {
        let c: Config = toml::from_str("picker = true").expect("parse");
        assert!(c.picker, "explicit field honored");
        assert!(c.paint, "missing field falls back to default");
        assert_eq!(c.pause_ms, DEFAULT_PAUSE_MS);
    }

    #[test]
    fn unknown_field_is_an_error() {
        // `deny_unknown_fields` keeps typos from silently no-op'ing —
        // a user who writes `picker_enabled = true` should see the
        // parse fail rather than wonder why nothing changed.
        let res: Result<Config, _> = toml::from_str("picker_enabled = true");
        assert!(res.is_err(), "unknown field should be rejected: {res:?}");
    }

    #[test]
    fn grammar_env_override_takes_precedence_over_file() {
        // Even if the config file says grammar = false, TUIPO_GRAMMAR=1
        // turns it on. Matches the env-wins-over-file precedence used by
        // every other setting.
        let c = Config {
            grammar: false,
            ..Config::default()
        };
        unsafe { std::env::set_var("TUIPO_GRAMMAR", "1") };
        let on = c.grammar_enabled();
        unsafe { std::env::remove_var("TUIPO_GRAMMAR") };
        assert!(on, "env should force grammar on");
    }

    #[test]
    fn picker_implies_tab_fix() {
        // Picker only opens via Tab, so enabling it must light up the
        // Tab-intercept path regardless of `tab_fix`.
        let c = Config {
            picker: true,
            ..Config::default()
        };
        assert!(c.tab_fix_enabled(), "picker on implies tab_fix on");
    }

    #[test]
    fn toml_roundtrip_preserves_fields() {
        // The config TUI writes back via `to_toml_string`; we then read
        // back via the same parser used at startup. The roundtrip must
        // be value-preserving or settings would silently drift.
        let original = Config {
            paint: false,
            underline: UnderlineMode::Fancy,
            tab_fix: true,
            picker: true,
            status_row: true,
            grammar: true,
            pause_ms: 200,
            hover_ms: 400,
            dict_path: Some("~/custom/dict.txt".into()),
            ..Config::default()
        };
        let serialized = original.to_toml_string();
        let parsed: Config = toml::from_str(&serialized).expect("roundtrip parse");
        assert_eq!(parsed.paint, original.paint);
        assert_eq!(parsed.underline, original.underline);
        assert_eq!(parsed.tab_fix, original.tab_fix);
        assert_eq!(parsed.picker, original.picker);
        assert_eq!(parsed.status_row, original.status_row);
        assert_eq!(parsed.grammar, original.grammar);
        assert_eq!(parsed.pause_ms, original.pause_ms);
        assert_eq!(parsed.hover_ms, original.hover_ms);
        assert_eq!(parsed.dict_path, original.dict_path);
    }

    #[test]
    fn toml_omits_dict_path_when_none() {
        let c = Config {
            dict_path: None,
            ..Config::default()
        };
        let s = c.to_toml_string();
        assert!(
            !s.contains("dict_path"),
            "dict_path should be absent when None: {s}"
        );
        // Roundtrip still works.
        let parsed: Config = toml::from_str(&s).expect("parse");
        assert!(parsed.dict_path.is_none());
    }

    #[test]
    fn toml_escapes_quotes_and_backslashes_in_dict_path() {
        let c = Config {
            dict_path: Some(r#"with "quote" and \backslash"#.into()),
            ..Config::default()
        };
        let s = c.to_toml_string();
        let parsed: Config = toml::from_str(&s).expect("parse escaped path");
        assert_eq!(parsed.dict_path.as_deref(), c.dict_path.as_deref());
    }
}

#[cfg(test)]
mod bilingual_config_tests {
    use super::*;
    #[test]
    fn bilingual_config_roundtrip_and_keys() {
        let cfg: Config = toml::from_str("languages = ['ru_RU']\nnext_key = 'ctrl+j'\nprevious_key = 'ctrl+k'\nrussian_aff_path = '/tmp/ru.aff'\nrussian_dic_path = '/tmp/ru.dic'\n").unwrap();
        let parsed: Config = toml::from_str(&cfg.to_toml_string()).unwrap();
        assert_eq!(parsed.languages, ["ru_RU"]);
        assert_eq!(parsed.russian_dic_path, cfg.russian_dic_path);
        assert_eq!(parsed.completion_direction(10), Some(1));
        assert_eq!(parsed.completion_direction(11), Some(-1));
        assert_eq!(parsed.completion_direction(14), None);
        let cfg = Config {
            highlight_only: true,
            tab_fix: true,
            picker: true,
            ..cfg
        };
        assert_eq!(cfg.completion_direction(10), None);
        assert!(!cfg.tab_fix_enabled());
        assert!(!cfg.picker_enabled());
    }
}
