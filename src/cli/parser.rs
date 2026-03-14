//! Parser - turning strings into sadness

/// Parse an address from various formats
pub fn parse_address(s: &str) -> Option<u64> {
    let s = s.trim();
    
    // Handle hex with 0x prefix
    if s.starts_with("0x") || s.starts_with("0X") {
        return u64::from_str_radix(&s[2..], 16).ok();
    }
    
    // Handle pure hex (if all chars are hex digits)
    if s.chars().all(|c| c.is_ascii_hexdigit()) && s.len() > 0 {
        return u64::from_str_radix(s, 16).ok();
    }
    
    // Try decimal
    s.parse::<u64>().ok()
}

/// Parse a size argument (for commands like pd, px)
pub fn parse_size(args: &[&str], default: usize) -> usize {
    args.first()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default)
}

/// Parse a command with modifiers (j for JSON, * for iterate, etc)
#[allow(dead_code)]
pub fn parse_command_modifiers(cmd: &str) -> (String, Vec<char>) {
    let mut base_cmd = String::new();
    let mut modifiers = Vec::new();
    
    for (i, c) in cmd.chars().enumerate() {
        if i > 0 && (c == 'j' || c == '*' || c == 'q' || c == '.') {
            modifiers.push(c);
        } else {
            base_cmd.push(c);
        }
    }
    
    (base_cmd, modifiers)
}

/// Check if command wants JSON output
#[allow(dead_code)]
pub fn wants_json(cmd: &str) -> bool {
    cmd.ends_with('j')
}

/// Check if command wants quiet output
#[allow(dead_code)]
pub fn wants_quiet(cmd: &str) -> bool {
    cmd.ends_with('q')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        assert_eq!(parse_address("0x1234"), Some(0x1234));
        assert_eq!(parse_address("0X1234"), Some(0x1234));
        assert_eq!(parse_address("deadbeef"), Some(0xdeadbeef));
        assert_eq!(parse_address("1234"), Some(0x1234)); // Ambiguous, treated as hex
        assert_eq!(parse_address(""), None);
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size(&["10"], 32), 10);
        assert_eq!(parse_size(&[], 32), 32);
        assert_eq!(parse_size(&["invalid"], 32), 32);
    }
}
