//! Parser 模块单元测试

#[cfg(test)]
mod tests {
    use codesage_core::{CodeSageError, Language};
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// 测试语言检测功能
    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
        assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
        assert_eq!(Language::from_extension("py"), Some(Language::Python));
        assert_eq!(Language::from_extension("unknown"), None);
    }

    /// 测试空文件解析
    #[test]
    fn test_empty_file() {
        let temp_file = NamedTempFile::with_suffix(".rs").unwrap();

        let parser = codesage_parser::CodeParser::new();
        let result = parser.parse_file(temp_file.path());

        assert!(
            result.is_ok(),
            "Failed to parse empty file: {:?}",
            result.err()
        );
        let parsed = result.unwrap();
        assert!(parsed.is_empty());
    }

    /// 测试解析源代码字符串
    #[test]
    fn test_parse_source() {
        let source = "fn test() { let x = 1; }";
        let parser = codesage_parser::CodeParser::new();
        let result = parser.parse_source(source, Language::Rust);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.language, Language::Rust);
        assert_eq!(parsed.source, source);
    }

    /// 测试行数计算
    #[test]
    fn test_line_count() {
        let source = "line1\nline2\nline3";
        let parser = codesage_parser::CodeParser::new();
        let parsed = parser.parse_source(source, Language::Rust).unwrap();

        assert_eq!(parsed.line_count(), 3);
    }
}
