//! Config - settings nobody will ever change

use std::collections::HashMap;

pub struct Config {
    pub settings: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        let mut settings = HashMap::new();
        
        // Default settings (mostly jokes)
        settings.insert("asm.arch".to_string(), "x86".to_string());
        settings.insert("asm.bits".to_string(), "64".to_string());
        settings.insert("asm.syntax".to_string(), "intel".to_string());
        settings.insert("asm.comments".to_string(), "true".to_string());
        settings.insert("asm.bytes".to_string(), "true".to_string());
        settings.insert("asm.sarcasm".to_string(), "maximum".to_string());
        settings.insert("scr.color".to_string(), "true".to_string());
        settings.insert("scr.utf8".to_string(), "true".to_string());
        settings.insert("anal.depth".to_string(), "42".to_string());
        settings.insert("anal.confidence".to_string(), "low".to_string());
        settings.insert("io.cache".to_string(), "true".to_string());
        settings.insert("cfg.fortunes".to_string(), "true".to_string());
        settings.insert("cfg.fortunes.type".to_string(), "all".to_string());
        settings.insert("dbg.backend".to_string(), "prayer".to_string());
        settings.insert("bin.demangle".to_string(), "try".to_string());
        settings.insert("bin.strings.min".to_string(), "4".to_string());
        settings.insert("user.sanity".to_string(), "questionable".to_string());
        
        Config { settings }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.settings.insert(key.to_string(), value.to_string());
    }

    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut items: Vec<_> = self.settings.iter().collect();
        items.sort_by(|a, b| a.0.cmp(b.0));
        items
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
