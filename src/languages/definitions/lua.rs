use crate::languages::Language;

pub struct Lua;

impl Language for Lua {
    fn name(&self) -> &'static str {
        "Lua"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["lua"]
    }

    fn line_comment(&self) -> Option<&'static str> {
        Some("--")
    }

    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        Some(("--[[", "]]"))
    }

    fn import_keywords(&self) -> &'static [&'static str] {
        &["require", "dofile", "loadfile"]
    }

    fn default_thresholds(&self) -> crate::Thresholds {
        crate::Thresholds {
            max_lines: 300,
            max_imports: 20,
            ..Default::default()
        }
    }
}
