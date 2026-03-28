use crate::languages::{Language, c_base::CBaseRules};

pub struct TypeScript;

impl Language for TypeScript {
    fn name(&self) -> &'static str {
        "TypeScript"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "tsx"]
    }
    fn line_comment(&self) -> Option<&'static str> {
        Some(CBaseRules::LINE_COMMENT)
    }
    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        Some(CBaseRules::BLOCK_COMMENT)
    }
    fn import_keywords(&self) -> &'static [&'static str] {
        &["import "]
    }
}
