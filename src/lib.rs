use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub max_items: usize,
    pub poll_interval_seconds: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            max_items: 50,
            poll_interval_seconds: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardStore {
    settings: AppSettings,
    items: Vec<String>,
}

impl Default for ClipboardStore {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            items: Vec::new(),
        }
    }
}

impl ClipboardStore {
    pub fn load() -> Self {
        let path = store_path();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(store) = serde_json::from_str::<ClipboardStore>(&content) {
                return store;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = store_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let payload = serde_json::to_string_pretty(self)?;
        fs::write(path, payload)?;
        Ok(())
    }

    pub fn add_clipboard_entry(&mut self, text: &str) {
        let value = text.trim();
        if value.is_empty() {
            return;
        }

        if let Some(pos) = self.items.iter().position(|x| x == value) {
            self.items.remove(pos);
        }

        self.items.insert(0, value.to_string());
        self.items.truncate(self.settings.max_items);
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    pub fn update_settings(&mut self, settings: AppSettings) {
        self.settings = settings;
        self.items.truncate(self.settings.max_items);
    }
}

fn store_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("usync-app").join("clipboard_store.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_store_unique_and_recent() {
        let mut store = ClipboardStore::default();
        store.add_clipboard_entry("one");
        store.add_clipboard_entry("two");
        store.add_clipboard_entry("one");
        assert_eq!(store.items(), &["one", "two"]);
    }

    #[test]
    fn settings_trim_items() {
        let mut store = ClipboardStore::default();
        for i in 0..5 {
            store.add_clipboard_entry(&format!("item-{i}"));
        }
        store.update_settings(AppSettings {
            max_items: 2,
            poll_interval_seconds: 1,
        });
        assert_eq!(store.items().len(), 2);
    }
}
