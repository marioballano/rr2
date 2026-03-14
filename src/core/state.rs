//! State - keeping track of our collective delusion

use std::path::PathBuf;
use crate::binary::loader::BinaryInfo;
use crate::core::config::Config;

#[allow(dead_code)]
pub struct ShellState {
    pub file_path: Option<PathBuf>,
    pub binary_info: Option<BinaryInfo>,
    pub current_address: u64,
    pub write_mode: bool,
    pub debug: bool,
    pub config: Config,
    pub analysis_done: bool,
    pub functions_detected: usize,
    pub strings_found: usize,
    pub xrefs_resolved: usize,
    pub history: Vec<String>,
}

impl ShellState {
    pub fn new(
        file_path: Option<PathBuf>,
        binary_info: Option<BinaryInfo>,
        write_mode: bool,
        debug: bool,
    ) -> Self {
        let entry_point = binary_info.as_ref()
            .map(|b| b.entry_point)
            .unwrap_or(0x00400000); // Classic default

        ShellState {
            file_path,
            binary_info,
            current_address: entry_point,
            write_mode,
            debug,
            config: Config::new(),
            analysis_done: false,
            functions_detected: 0,
            strings_found: 0,
            xrefs_resolved: 0,
            history: Vec::new(),
        }
    }

    pub fn seek(&mut self, addr: u64) {
        self.current_address = addr;
    }

    pub fn seek_relative(&mut self, offset: i64) {
        self.current_address = (self.current_address as i64 + offset) as u64;
    }

    pub fn has_file(&self) -> bool {
        self.file_path.is_some()
    }

    pub fn file_name(&self) -> &str {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("malloc://512")
    }
}
