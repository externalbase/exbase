use std::{collections::HashMap, ops::Range};
use crate::{MemoryAccessor};

pub enum ScanType {
    First,
    All
}

pub struct PatternScanner {
    patterns: HashMap<String, Pattern>,
    memory_range: Range<usize>,
}

pub struct Pattern {
    pattern: Vec<u8>,
    mask: Vec<u8>,
    scan_type: ScanType,
    event: fn(slice_out: &[u8]),
}

impl PatternScanner {
    pub fn new(memory_range: Range<usize>) -> Self {
        Self {
            patterns: HashMap::default(),
            memory_range
        }
    }

    pub fn add_pattern(&mut self, name: String, pattern: Pattern) {
        self.patterns.insert(name, pattern);
    }

    pub fn scan(&mut self, mem: impl MemoryAccessor) {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.memory_range.len());
        mem.read_buffer(&mut bytes, self.memory_range.start);
        todo!();
    }
}

impl Pattern {
    /// # Example
    /// 00 FF 0? F? ?? ?
    /// ```
    /// ```
    pub fn new(pattern: &str, scan_type: ScanType, event: fn(slice_out: &[u8])) -> Option<Self> {
        let mut result = Pattern { event, scan_type, pattern: Vec::new(), mask: Vec::new() };
        let mut first_solid_found = false;
        if pattern.trim().is_empty() {
            return  None;
        }
        for strb in pattern.split_whitespace() {
            if strb.len() > 2 {
                return None;
            }
            let any_wild = strb.contains('?');
            let full_wild = strb == "??" || (any_wild && strb.len() == 1);

            if !first_solid_found && !full_wild {
                first_solid_found = true;
            }
            if full_wild {
                result.pattern.push(0x00);
                result.mask.push(0x00);
                continue;
            }

            if !any_wild {
                let byte = u8::from_str_radix(strb, 16).ok()?;
                result.pattern.push(byte);
                result.mask.push(0xFF);
                continue;
            }

            let mut chars: Vec<char> = strb.chars().collect();
            if chars[0] == '?' {
                chars[0] = '0';
                let hex = chars.iter().collect::<String>();
                let byte = u8::from_str_radix(&hex, 16).ok()?;
                result.pattern.push(byte);
                result.mask.push(0x0F);
            } else {
                chars[1] = '0';
                let hex = chars.iter().collect::<String>();
                let byte = u8::from_str_radix(&hex, 16).ok()?;
                result.pattern.push(byte);
                result.mask.push(0xF0);
            }
        }

        Some(result)
    }
}