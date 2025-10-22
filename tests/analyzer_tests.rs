//! Analyzer 模块单元测试

#[cfg(test)]
mod tests {
    use codesage_core::{AnalysisContext, Analyzer, Language};
    use std::path::PathBuf;

    /// 测试圈复杂度计算
    #[test]
    fn test_cyclomatic_complexity() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();

        // 简单函数，复杂度为1（基本路径）
        let simple_code = "fn simple() { let x = 1; }";
        let metrics = analyzer.calculate_metrics(simple_code);
        assert_eq!(metrics.cyclomatic_complexity, 1);

        // 包含 if 的函数，复杂度为2
        let if_code = r#"
        fn with_if(x: i32) {
            if x > 0 {
                println!("positive");
            } else {
                println!("non-positive");
            }
        }"#;
        let metrics = analyzer.calculate_metrics(if_code);
        // 根据当前算法，复杂度为2（基本路径+if）
        assert!(metrics.cyclomatic_complexity >= 2);

        // 更复杂的函数，包含多个决策点
        let complex_code = r#"
        fn complex(x: i32, y: i32) {
            if x > 0 {
                for i in 0..x {
                    if y > 0 {
                        println!("both positive");
                    }
                }
            }
        }"#;
        let metrics = analyzer.calculate_metrics(complex_code);
        // 根据当前算法，至少包含if、for、内部if
        assert!(metrics.cyclomatic_complexity >= 3);
    }

    /// 测试认知复杂度计算
    #[test]
    fn test_cognitive_complexity() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();

        // 简单函数，复杂度为0
        let simple_code = "fn simple() { let x = 1; }";
        let metrics = analyzer.calculate_metrics(simple_code);
        assert_eq!(metrics.cognitive_complexity, 0);

        // 包含嵌套的函数
        let nested_code = r#"
        fn nested() {
            if true {
                if true {
                    if true {
                        println!("deep");
                    }
                }
            }
        }"#;
        let metrics = analyzer.calculate_metrics(nested_code);
        // 根据当前算法，嵌套会增加复杂度
        assert!(metrics.cognitive_complexity > 0);
    }

    /// 测试代码重复率检测
    #[test]
    fn test_duplication_detection() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();

        // 无重复代码
        let unique_code = r#"
        fn func1() { println!("one"); }
        fn func2() { println!("two"); }
        "#;
        let metrics = analyzer.calculate_metrics(unique_code);
        assert!(metrics.duplication_percentage < 1.0); // 应该非常低

        // 包含重复代码
        let duplicate_code = r#"
        fn func1() { 
            println!("duplicate"); 
            println!("duplicate"); 
        }
        fn func2() { 
            println!("duplicate"); 
            println!("duplicate"); 
        }
        "#;
        let metrics = analyzer.calculate_metrics(duplicate_code);
        assert!(metrics.duplication_percentage > 0.0);
    }

    /// 测试分析器接口
    #[test]
    fn test_analyzer_trait() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();
        assert_eq!(analyzer.name(), "metrics");

        let context = AnalysisContext {
            file_path: PathBuf::from("test.rs"),
            source_code: "fn test() { let x = 1; }".to_string(),
            language: Language::Rust,
        };

        let result = analyzer.analyze(&context);
        assert!(result.is_ok());

        let issues = result.unwrap();
        // 简单函数不应该产生复杂度问题
        let high_complexity_issues: Vec<_> = issues
            .iter()
            .filter(|issue| issue.message.contains("complexity"))
            .collect();
        assert!(high_complexity_issues.is_empty());
    }

    /// 测试高复杂度代码检测
    #[test]
    fn test_high_complexity_detection() {
        let analyzer = codesage_analyzer::MetricsAnalyzer::new();

        let complex_code = r#"fn complex() {
            if true {
                if true {
                    if true {
                        if true {
                            if true {
                                println!("very complex");
                            }
                        }
                    }
                }
            }
        }"#;
        let context = AnalysisContext {
            file_path: PathBuf::from("complex.rs"),
            source_code: complex_code.to_string(),
            language: Language::Rust,
        };

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

    /// 测试分析引擎
    #[test]
    fn test_analysis_engine() {
        let mut engine = codesage_analyzer::AnalysisEngine::new();
        engine.register_analyzer(Box::new(codesage_analyzer::MetricsAnalyzer::new()));

        let context = AnalysisContext {
            file_path: PathBuf::from("test.rs"),
            source_code: "fn test() { let x = 1; }".to_string(),
            language: Language::Rust,
        };

        let result = engine.analyze(&context);
        assert!(result.is_ok());

        let issues = result.unwrap();
        // 简单代码不应产生太多问题
        assert!(issues.len() < 5);
    }
}
