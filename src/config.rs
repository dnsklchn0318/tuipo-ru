//! Read-only configuration for explicit file checking.
use serde::Deserialize;
use std::{path::PathBuf, sync::OnceLock};
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub languages: Vec<String>,
    pub grammar: bool,
    pub dict_path: Option<String>,
    pub russian_aff_path: Option<String>,
    pub russian_dic_path: Option<String>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            languages: vec!["ru_RU".into(), "en_US".into()],
            grammar: false,
            dict_path: None,
            russian_aff_path: None,
            russian_dic_path: None,
        }
    }
}
impl Config {
    fn load() -> Result<Self, &'static str> {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".config")));
        let cfg: Self = if let Some(base) = base {
            match std::fs::read_to_string(base.join("tuipo/config.toml")) {
                Ok(text) => toml::from_str(&text)
                    .map_err(|_| "invalid configuration (contents suppressed)")?,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Self::default(),
                Err(_) => return Err("configuration unavailable"),
            }
        } else {
            Self::default()
        };
        if cfg.languages.is_empty()
            || cfg
                .languages
                .iter()
                .any(|l| !matches!(l.as_str(), "ru_RU" | "en_US"))
            || cfg.russian_aff_path.is_some() != cfg.russian_dic_path.is_some()
        {
            return Err("invalid spelling settings");
        }
        if let (Some(aff), Some(dic)) = (&cfg.russian_aff_path, &cfg.russian_dic_path) {
            let aff = std::fs::read_to_string(aff).map_err(|_| "dictionary unavailable")?;
            let dic = std::fs::read_to_string(dic).map_err(|_| "dictionary unavailable")?;
            spellbook::Dictionary::new(&aff, &dic).map_err(|_| "invalid dictionary")?;
        }
        Ok(cfg)
    }
}
static GLOBAL: OnceLock<Config> = OnceLock::new();
pub fn load_global() -> Result<(), &'static str> {
    let cfg = Config::load()?;
    let _ = GLOBAL.set(cfg);
    Ok(())
}
pub fn get() -> &'static Config {
    GLOBAL.get_or_init(Config::default)
}
