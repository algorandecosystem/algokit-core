use super::types::LogicError;
use super::{AppClient, AppSourceMaps};
use crate::transactions::TransactionResultError;
use serde_json::Value as JsonValue;

impl AppClient {
    pub fn import_source_maps(&mut self, source_maps: AppSourceMaps) {
        self.source_maps = Some(source_maps);
    }

    pub fn export_source_maps(&self) -> Option<AppSourceMaps> {
        self.source_maps.clone()
    }
}

impl AppClient {
    /// Create an enhanced LogicError from a transaction error, applying source maps if available.
    pub fn expose_logic_error(
        &self,
        error: &TransactionResultError,
        is_clear_state_program: bool,
    ) -> LogicError {
        let err_str = format!("{}", error);
        let (line_no_opt, listing) =
            self.apply_source_map_for_message(&err_str, is_clear_state_program);
        let source_map = self.get_source_map(is_clear_state_program).cloned();
        let transaction_id = Self::extract_transaction_id(&err_str);

        let logic = LogicError {
            logic_error_str: err_str.clone(),
            program: None,
            source_map,
            transaction_id,
            pc: Self::extract_pc(&err_str),
            line_no: line_no_opt,
            lines: if listing.is_empty() {
                None
            } else {
                Some(listing)
            },
            traces: None,
        };

        if crate::config::Config::debug() {
            // TODO: Add traces to LogicError
        }

        logic
    }

    fn extract_transaction_id(error_str: &str) -> Option<String> {
        let re = regex::Regex::new(r"transaction ([A-Z2-7]{52})").unwrap();
        re.captures(error_str)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn apply_source_map_for_message(
        &self,
        error_str: &str,
        is_clear_state_program: bool,
    ) -> (Option<u64>, Vec<String>) {
        let pc_opt = Self::extract_pc(error_str);
        if let Some(pc) = pc_opt {
            if let Some((line_no, listing)) = self.apply_source_map(pc, is_clear_state_program) {
                return (Some(line_no), listing);
            }
        }
        (None, Vec::new())
    }

    fn extract_pc(s: &str) -> Option<u64> {
        for token in s.split(|c: char| c.is_whitespace() || c == ',' || c == ';') {
            if let Some(idx) = token.find('=') {
                let (k, v) = token.split_at(idx);
                if k.ends_with("pc") {
                    if let Ok(parsed) = v.trim_start_matches('=').parse::<u64>() {
                        return Some(parsed);
                    }
                }
            }
        }
        None
    }

    fn apply_source_map(
        &self,
        pc: u64,
        is_clear_state_program: bool,
    ) -> Option<(u64, Vec<String>)> {
        let map = self.get_source_map(is_clear_state_program)?;
        let line_no = Self::map_pc_to_line(map, pc)?;
        let listing = Self::truncate_listing(map, line_no, 3);
        Some((line_no, listing))
    }

    fn get_source_map(&self, is_clear_state_program: bool) -> Option<&JsonValue> {
        let maps = self.source_maps.as_ref()?;
        if is_clear_state_program {
            maps.clear_source_map.as_ref()
        } else {
            maps.approval_source_map.as_ref()
        }
    }

    fn map_pc_to_line(map: &JsonValue, pc: u64) -> Option<u64> {
        let pcs = map.get("pc")?.as_array()?;
        let mut best_line: Option<u64> = None;
        for (i, entry) in pcs.iter().enumerate() {
            if let Some(pc_val) = entry.as_u64() {
                if pc_val == pc {
                    return Some(i as u64 + 1);
                }
                if pc_val < pc {
                    best_line = Some(i as u64 + 1);
                }
            }
        }
        best_line
    }

    /// Extracts and formats a code snippet from source code around a specific line with context.
    ///
    /// Given a JSON object containing source code, extracts a window of lines centered around
    /// `center_line` with `context` lines above and below. Returns formatted lines with
    /// line numbers for error display purposes.
    fn truncate_listing(map: &JsonValue, center_line: u64, context: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        if let Some(source) = map.get("source").and_then(|s| s.as_str()) {
            let src_lines: Vec<&str> = source.lines().collect();
            let total = src_lines.len();
            let center = center_line.saturating_sub(1) as usize;
            let start = center.saturating_sub(context);
            let end = (center + context + 1).min(total);
            for (i, line) in src_lines.iter().enumerate().take(end).skip(start) {
                lines.push(format!("{:>4} | {}", i + 1, line));
            }
        }
        lines
    }
}
