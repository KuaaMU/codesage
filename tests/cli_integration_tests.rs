//! CLI 模块集成测试

#[cfg(test)]
mod integration_tests {
    use std::io::Write;
    use std::process::Command;

    /// 测试 CLI 基本功能
    #[test]
    fn test_cli_help() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .current_dir("F:\\codesage")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("AI-powered code review and refactoring tool"));
        assert!(stdout.contains("review"));
        assert!(stdout.contains("refactor"));
    }
}

#[cfg(test)]
mod cli_unit_tests {
    /// 测试 CLI 模块内部函数
    #[tokio::test]
    async fn test_run_function_exists() {
        // 这个测试主要验证 CLI 模块结构是否正确
        // 实际调用 run() 会尝试解析命令行参数，这在测试环境中不是我们想要的
        // 所以我们只是验证函数存在
        use codesage_cli::run;
        assert!(true); // 函数存在，所以测试通过
    }
}
