use crate::languages::{Language, c_base::CBaseRules};

pub struct Cpp;

impl Language for Cpp {
    fn name(&self) -> &'static str {
        "C/C++"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["cpp", "c", "h", "hpp", "cc", "cxx", "hh", "hpp", "hxx"]
    }

    fn line_comment(&self) -> Option<&'static str> {
        Some(CBaseRules::LINE_COMMENT)
    }

    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        Some(CBaseRules::BLOCK_COMMENT)
    }

    fn import_keywords(&self) -> &'static [&'static str] {
        &["#include "]
    }

    fn default_thresholds(&self) -> crate::Thresholds {
        crate::Thresholds {
            max_lines: 500,
            max_imports: 30,
            max_depth: 7,
            ..Default::default()
        }
    }
}
