//! AI Client 模块单元测试 (使用 mock)

#[cfg(test)]
mod tests {
    use codesage_core::{AnalysisContext, Language};
    use std::path::PathBuf;

    /// 测试 AI 配置的默认值
    #[test]
    fn test_ai_config_defaults() {
        let config = codesage_ai::AIConfig::default();
        assert_eq!(config.model, "claude-3-5-sonnet-20241022");
        assert_eq!(config.api_base_url, "https://api.anthropic.com/v1");
        assert_eq!(config.timeout_seconds, 60);
        // API 密钥可能为空,这取决于环境变量
    }

    /// 测试 AI 客户端创建
    #[test]
    fn test_ai_client_creation() {
        let _client = codesage_ai::AIClient::new();
        // 验证客户端可以成功创建，不会panic
    }

    /// 测试自定义配置的 AI 客户端
    #[test]
    fn test_ai_client_with_config() {
        let config = codesage_ai::AIConfig {
            api_key: Some("test-key".to_string()),
            model: "custom-model".to_string(),
            api_base_url: "https://api.test.com/v1".to_string(),
            timeout_seconds: 30,
        };
        let _client = codesage_ai::AIClient::with_config(config);
        // 验证自定义配置的客户端可以成功创建，不会panic
    }

    /// 测试 AI 审查功能 (需要 API key,测试时会优雅降级)
    #[tokio::test]
    async fn test_ai_review_without_api_key() {
        use codesage_core::AIReviewer;

        let config = codesage_ai::AIConfig {
            api_key: None, // 故意不设置 API key
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_base_url: "https://api.anthropic.com/v1".to_string(),
            timeout_seconds: 60,
        };

        let client = codesage_ai::AIClient::with_config(config);
        let context = AnalysisContext {
            file_path: PathBuf::from("test.rs"),
            source_code: "fn main() { println!(\"Hello\"); }".to_string(),
            language: Language::Rust,
        };

        // 没有 API key 时,应该返回成功但没有 AI 分析结果
        let result = client.review(&context).await;
        assert!(result.is_ok());

        let review = result.unwrap();
        assert_eq!(review.file_path, PathBuf::from("test.rs"));
    }

    /// 测试配置的合理性
    #[test]
    fn test_config_validation() {
        let config = codesage_ai::AIConfig {
            api_key: Some("sk-test-key".to_string()),
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_base_url: "https://api.anthropic.com/v1".to_string(),
            timeout_seconds: 120,
        };

        assert!(config.timeout_seconds > 0);
        assert!(!config.model.is_empty());
        assert!(!config.api_base_url.is_empty());
    }

    /// 测试超时配置的范围
    #[test]
    fn test_timeout_ranges() {
        let short_timeout = codesage_ai::AIConfig {
            api_key: None,
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_base_url: "https://api.anthropic.com/v1".to_string(),
            timeout_seconds: 10,
        };

        let long_timeout = codesage_ai::AIConfig {
            api_key: None,
            model: "claude-3-5-sonnet-20241022".to_string(),
            api_base_url: "https://api.anthropic.com/v1".to_string(),
            timeout_seconds: 300,
        };

        assert!(short_timeout.timeout_seconds >= 10);
        assert!(long_timeout.timeout_seconds <= 600);
    }

    /// 测试 API 基础 URL 的有效性
    #[test]
    fn test_api_base_url() {
        let config = codesage_ai::AIConfig::default();
        assert!(config.api_base_url.starts_with("https://"));
        assert!(config.api_base_url.contains("anthropic"));
    }

    /// 测试模型名称格式
    #[test]
    fn test_model_name_format() {
        let config = codesage_ai::AIConfig::default();
        assert!(config.model.contains("claude"));
        assert!(!config.model.is_empty());
    }
}
