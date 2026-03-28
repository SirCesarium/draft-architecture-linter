use crate::languages::{Language, c_base::CBaseRules};

pub struct Rust;

impl Language for Rust {
    fn name(&self) -> &'static str {
        "Rust"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }
    fn line_comment(&self) -> Option<&'static str> {
        Some(CBaseRules::LINE_COMMENT)
    }
    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        Some(CBaseRules::BLOCK_COMMENT)
    }
    fn import_keywords(&self) -> &'static [&'static str] {
        &["use "]
    }
}
