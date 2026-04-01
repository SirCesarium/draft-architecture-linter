//! Language Extensibility System.
//!
//! @swt-disable max-repetition
//!
//! This module implements the Strategy pattern to define language-specific
//! analysis rules. To add support for a new language:
//!
//! 1. Create a new struct in `definitions/`.
//! 2. Implement the `Language` trait for it (usually via `define_language!`).
//! 3. Register the new struct in `LanguageRegistry::new()`.

pub mod c_base;
pub mod definitions;

use std::collections::HashMap;
use std::sync::OnceLock;

/// Interface for language-specific analysis strategies.
///
/// Defines the syntax rules (comments, imports) and thresholds
/// required for analyzing a specific programming language.
pub trait Language: Send + Sync {
    /// Friendly name of the language (e.g., "Rust").
    fn name(&self) -> &'static str;

    /// File extensions associated with this language (without the dot).
    fn extensions(&self) -> &'static [&'static str];

    /// Delimiter for single-line comments (e.g., Some("//")).
    fn line_comment(&self) -> Option<&'static str>;

    /// Start and end delimiters for multi-line block comments (e.g., Some(("/*", "*/"))).
    fn block_comment(&self) -> Option<(&'static str, &'static str)>;

    /// Keywords used to declare imports or dependencies (e.g., &[`use`, `import`]).
    fn import_keywords(&self) -> &'static [&'static str];

    /// Number of spaces representing one level of indentation. Defaults to 4.
    fn indent_size(&self) -> usize {
        4
    }

    /// Default health thresholds specifically tuned for this language.
    fn default_thresholds(&self) -> crate::Thresholds {
        crate::Thresholds::default()
    }
}

/// Helper macro to define new languages with minimal boilerplate.
#[macro_export]
macro_rules! define_language {
    (
        $struct_name:ident,
        $display_name:expr,
        extensions: [$($ext:expr),*],
        line_comment: $line_comment:expr,
        block_comment: $block_comment:expr,
        import_keywords: [$($kw:expr),*]
        $(, thresholds: $thresholds:expr )?
    ) => {
        pub struct $struct_name;

        impl $crate::languages::Language for $struct_name {
            fn name(&self) -> &'static str { $display_name }
            fn extensions(&self) -> &'static [&'static str] { &[$($ext),*] }
            fn line_comment(&self) -> Option<&'static str> { $line_comment }
            fn block_comment(&self) -> Option<(&'static str, &'static str)> { $block_comment }
            fn import_keywords(&self) -> &'static [&'static str] { &[$($kw),*] }
            $(
                fn default_thresholds(&self) -> $crate::Thresholds { $thresholds }
            )?
        }
    };
}

/// Thread-safe registry for managing supported languages.
pub struct LanguageRegistry {
    languages: Vec<Box<dyn Language>>,
    extension_map: HashMap<&'static str, usize>,
}

static REGISTRY: OnceLock<LanguageRegistry> = OnceLock::new();

impl LanguageRegistry {
    /// Return the global registry instance.
    #[must_use]
    pub fn get() -> &'static Self {
        REGISTRY.get_or_init(Self::new)
    }

    fn new() -> Self {
        let languages: Vec<Box<dyn Language>> = vec![
            Box::new(definitions::rust::Rust),
            Box::new(definitions::python::Python),
            Box::new(definitions::javascript::JavaScript),
            Box::new(definitions::typescript::TypeScript),
            Box::new(definitions::java::Java),
            Box::new(definitions::csharp::CSharp),
            Box::new(definitions::gdscript::GDScript),
            Box::new(definitions::lua::Lua),
            Box::new(definitions::go::Go),
            Box::new(definitions::php::PHP),
            Box::new(definitions::cpp::Cpp),
        ];

        let mut extension_map = HashMap::new();
        for (i, lang) in languages.iter().enumerate() {
            for ext in lang.extensions() {
                extension_map.insert(*ext, i);
            }
        }

        Self {
            languages,
            extension_map,
        }
    }

    /// Resolve a language strategy by file extension.
    #[must_use]
    pub fn get_by_extension(&self, ext: &str) -> Option<&dyn Language> {
        self.extension_map
            .get(ext)
            .map(|&i| self.languages[i].as_ref())
    }

    /// Return a list of all supported file extensions.
    #[must_use]
    pub fn supported_extensions(&self) -> Vec<&'static str> {
        let mut extensions: Vec<&'static str> = self.extension_map.keys().copied().collect();
        extensions.sort_unstable();
        extensions
    }
}
