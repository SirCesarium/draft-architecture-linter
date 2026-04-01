use crate::languages::Language;

pub struct Go;

impl Language for Go {
    fn name(&self) -> &'static str {
        "Go"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["go"]
    }

    fn line_comment(&self) -> Option<&'static str> {
        Some("//")
    }

    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        Some(("/*", "*/"))
    }

    fn import_keywords(&self) -> &'static [&'static str] {
        &["import", "import ("]
    }

    fn default_thresholds(&self) -> crate::Thresholds {
        crate::Thresholds {
            max_lines: 400,
            max_imports: 20,
            ..Default::default()
        }
    }
}
