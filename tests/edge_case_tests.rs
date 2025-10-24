//! 边界条件测试

#[cfg(test)]
mod edge_case_tests {
    use codesage_core::{AnalysisContext, Analyzer, Language};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    /// 测试超大文件的行数计算
    #[test]
    fn test_large_file_line_count() {
        // 创建一个包含很多行的字符串
        let large_content = (0..1000)
            .map(|i| format!("line {}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let parser = codesage_parser::CodeParser::new();
        let parsed = parser.parse_source(&large_content, Language::Rust).unwrap();

        assert_eq!(parsed.line_count(), 1000);
    }

    /// 测试空字符串
    #[test]
    fn test_empty_string() {
        let parser = codesage_parser::CodeParser::new();
        let parsed = parser.parse_source("", Language::Rust).unwrap();

        assert!(parsed.is_empty());
        assert_eq!(parsed.line_count(), 0);
    }

    /// 测试只包含空白字符的字符串
    #[test]
    fn test_whitespace_only() {
        let parser = codesage_parser::CodeParser::new();
        let parsed = parser
            .parse_source("   \n\t\n  \n", Language::Rust)
            .unwrap();

        // is_empty() 检查去除空白后是否为空
        assert!(parsed.is_empty()); // 只有空白字符应该返回 true
        assert_eq!(parsed.line_count(), 3);
    }

    /// 测试包含特殊字符的代码
    #[test]
    fn test_special_characters() {
        let special_code = r#"fn special() {
            let s = "Hello, 世界! 🌍";
            let c = '\u{1F600}'; // Grinning face emoji
            println!("Special chars: {}, {}", s, c);
        }"#;

        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        let metrics = analyzer.calculate_metrics(special_code);

        // 特殊字符不应影响基本指标计算
        assert!(metrics.lines_of_code > 0);
        assert!(metrics.cyclomatic_complexity >= 1); // 至少有基本复杂度
    }

    /// 测试极复杂代码的处理
    #[test]
    fn test_extremely_complex_code() {
        // 创建一个嵌套层次极深的代码片段
        let mut complex_code = String::from("fn deep() {\n");
        for _ in 0..50 {
            complex_code.push_str(" if true {\n");
        }
        complex_code.push_str("println!(\"inner\");\n");
        for _ in 0..50 {
            complex_code.push_str(" }\n");
        }
        complex_code.push('}');

        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        let metrics = analyzer.calculate_metrics(&complex_code);

        // 复杂代码应该有较高的复杂度值
        assert!(metrics.cyclomatic_complexity > 10);
        // 认知复杂度也应该很高(至少有一些嵌套)
        assert!(metrics.cognitive_complexity > 0);
    }

    /// 测试零除和数值边界
    #[test]
    fn test_zero_division_safety() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        // 空代码不应导致除零错误
        let metrics = analyzer.calculate_metrics("");

        // 检查指标是否有效
        assert_eq!(metrics.lines_of_code, 0);
        assert!(metrics.maintainability_index >= 0.0);
        assert!(metrics.maintainability_index <= 100.0);
    }

    /// 测试超长行的处理
    #[test]
    fn test_very_long_line() {
        let very_long_line = format!("fn test() {{ let x = {}; }}", "a".repeat(10000));

        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        let metrics = analyzer.calculate_metrics(&very_long_line);

        assert!(metrics.lines_of_code > 0);
        // 应该成功处理而不崩溃
    }

    /// 测试文件解析边界情况
    #[test]
    fn test_file_edge_cases() {
        // 测试包含特殊字符的文件名

        let mut temp_file = NamedTempFile::with_suffix(".rs").unwrap();
        writeln!(temp_file, "fn test() {{ println!(\"hello\"); }}").unwrap();
        temp_file.flush().unwrap();

        let parser = codesage_parser::CodeParser::new();
        let result = parser.parse_file(temp_file.path());

        // 应该成功解析临时文件
        assert!(result.is_ok(), "Failed to parse file: {:?}", result.err());
        let parsed = result.unwrap();
        assert_eq!(parsed.language, Language::Rust);
    }

    /// 测试复杂度阈值边界
    #[test]
    fn test_complexity_thresholds() {
        // 创建复杂度刚好达到或超过阈值的代码
        let high_complexity_code = r#"fn high_complexity() {
            if true {} // 1
            if true {} // 2
            if true {} // 3
            if true {} // 4
            if true {} // 5
            if true {} // 6
            if true {} // 7
            if true {} // 8
            if true {} // 9
            if true {} // 10
            if true {} // 11 - 超过阈值
        }"#;

        let context = AnalysisContext {
            file_path: PathBuf::from("high_complexity.rs"),
            source_code: high_complexity_code.to_string(),
            language: Language::Rust,
        };

        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        let result = analyzer.analyze(&context);

        assert!(result.is_ok());
        let issues = result.unwrap();

        // 高复杂度代码应该产生问题
        let complexity_issues: Vec<_> = issues
            .iter()
            .filter(|issue| issue.message.contains("complexity"))
            .collect();
        assert!(!complexity_issues.is_empty());
    }

    /// 测试分析上下文边界
    #[test]
    fn test_analysis_context_edge_cases() {
        // 测试路径包含特殊字符
        let context = AnalysisContext {
            file_path: PathBuf::from("path/with/special_chars_123.rs"),
            source_code: "fn test() {}".to_string(),
            language: Language::Rust,
        };

        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        let result = analyzer.analyze(&context);

        assert!(result.is_ok());
    }
}
