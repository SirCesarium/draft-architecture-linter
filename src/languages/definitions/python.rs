use crate::languages::Language;

pub struct Python;

impl Language for Python {
    fn name(&self) -> &'static str {
        "Python"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["py"]
    }
    fn line_comment(&self) -> Option<&'static str> {
        Some("#")
    }
    fn block_comment(&self) -> Option<(&'static str, &'static str)> {
        None
    }
    fn import_keywords(&self) -> &'static [&'static str] {
        &["import ", "from "]
    }
}
