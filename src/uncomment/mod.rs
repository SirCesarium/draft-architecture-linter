mod handlers;
mod utils;

use crate::languages::LanguageRegistry;
use handlers::{handle_block_comment_end, handle_comment_start, try_handle_jsx_comment};
use utils::normalize_whitespace;

#[must_use]
pub fn remove_comments(content: &str, extension: &str, aggressive: bool) -> String {
    let registry = LanguageRegistry::get();
    let Some(lang) = registry.get_by_extension(extension) else {
        return content.to_string();
    };

    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars();
    let line_prefix = lang.line_comment();
    let block_delimiters = lang.block_comment();

    let mut in_string = false;
    let mut string_char = '\"';
    let mut in_block_comment = false;
    let mut in_line_comment = false;

    while let Some(current) = chars.next() {
        if in_string {
            result.push(current);
            if current == '\\' {
                if let Some(escaped) = chars.next() {
                    result.push(escaped);
                }
            } else if current == string_char {
                in_string = false;
            }
            continue;
        }

        if in_block_comment {
            if handle_block_comment_end(
                &mut chars,
                &mut result,
                &mut in_block_comment,
                block_delimiters,
                current,
                extension,
            ) {
                continue;
            }
            continue;
        }

        if in_line_comment {
            if current == '\n' {
                in_line_comment = false;
                result.push('\n');
            }
            continue;
        }

        if current == '\"' || current == '\'' || current == '`' {
            in_string = true;
            string_char = current;
            result.push(current);
            continue;
        }

        if (extension == "jsx" || extension == "tsx")
            && current == '{'
            && try_handle_jsx_comment(&mut chars, &mut in_block_comment)
        {
            continue;
        }

        if handle_comment_start(
            &mut chars,
            &mut in_block_comment,
            &mut in_line_comment,
            block_delimiters,
            line_prefix,
            current,
            aggressive,
        ) {
            continue;
        }

        result.push(current);
    }

    normalize_whitespace(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(code: &str, expected: &str, aggressive: bool) {
        assert_eq!(remove_comments(code, "rs", aggressive), expected);
    }

    #[test]
    fn test_uncommenting() {
        check(
            "fn main() {\n    // comment\n    /* block */\n    let x = 5;\n}",
            "fn main() {\n\n    let x = 5;\n}",
            true,
        );
        check("/// doc\nfn main() {}", "/// doc\nfn main() {}", false);
        check("/// doc\nfn main() {}", "fn main() {}", true);
        check(
            "let s = \"http://example.com\";",
            "let s = \"http://example.com\";",
            true,
        );
    }

    #[test]
    fn test_jsx_uncommenting() {
        assert_eq!(
            remove_comments("const el = <div>{/* comment */}</div>", "tsx", true),
            "const el = <div></div>"
        );
        assert_eq!(
            remove_comments("const el = <div>{  /* spaces */  }</div>", "tsx", true),
            "const el = <div></div>"
        );
        assert_eq!(
            remove_comments("const el = <div>{\n  /* multiline */\n}</div>", "tsx", true),
            "const el = <div></div>"
        );
    }
}
