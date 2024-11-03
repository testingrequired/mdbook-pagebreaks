use regex::{escape, Regex};

pub const PAGE_BREAK: &'static str = "{{---}}";
pub const HTML_BREAK: &'static str = "<div class=\"mdbook_pagebreak\">&nbsp;</div>";

/// Replace all instances of `{{---}}` with a replacement string
pub fn replace_page_breaks(input: &str, replacement: &str) -> String {
    let pattern_str = format!("(?m)^{}", escape(PAGE_BREAK));
    let pattern = Regex::new(&pattern_str).unwrap();

    let result = pattern.replace_all(input, replacement);

    result.to_string()
}

/// Replace all instances of `{{---}}` with HTML page break
pub fn replace_html_page_breaks(input: &str) -> String {
    replace_page_breaks(input, &HTML_BREAK)
}

/// Replace all instances of `{{---}}` with HTML page break
pub fn remove_page_breaks(input: &str) -> String {
    replace_page_breaks(input, "")
}

#[cfg(test)]
mod replace_html_page_breaks_tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_replace_bare_placeholder() {
        assert_eq!(HTML_BREAK, replace_html_page_breaks(&PAGE_BREAK));
    }

    #[test]
    fn should_replace_embedded_on_newlines_placeholder() {
        assert_eq!(
            format!(
                "# Title\nSome text\n{}\nMore text\n{}",
                &HTML_BREAK, &HTML_BREAK
            ),
            replace_html_page_breaks(&format!(
                "# Title\nSome text\n{}\nMore text\n{}",
                &PAGE_BREAK, &PAGE_BREAK
            ))
        );
    }

    #[test]
    fn should_not_replace_embedded_in_text_placeholder() {
        assert_eq!(
            format!(r#"Hello {} World"#, &PAGE_BREAK),
            replace_html_page_breaks(&format!(r#"Hello {} World"#, &PAGE_BREAK))
        );
    }
}

#[cfg(test)]
mod remove_page_breaks_tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_remove_bare_placeholder() {
        assert_eq!("", remove_page_breaks(&PAGE_BREAK));
    }

    #[test]
    fn should_replace_embedded_on_newlines_placeholder() {
        assert_eq!(
            "# Title\nSome text\n\nMore text\n",
            remove_page_breaks(&format!(
                "# Title\nSome text\n{}\nMore text\n{}",
                &PAGE_BREAK, &PAGE_BREAK
            ))
        );
    }

    #[test]
    fn should_not_replace_embedded_in_text_placeholder() {
        assert_eq!(
            format!(r#"Hello {} World"#, &PAGE_BREAK),
            remove_page_breaks(&format!(r#"Hello {} World"#, &PAGE_BREAK))
        );
    }
}
