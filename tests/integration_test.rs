use minigrep::{search, search_case_insensitive, Config};

/// -- 测试大小写敏感搜索功能
#[test]
fn test_case_sensitive() {
    let query = "duct";
    let contents = r#"
Rust:
safe, fast, productive.
Pick three.
Duct tape."#;

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

/// -- 测试大小写不敏感搜索功能
#[test]
fn test_case_insensitive() {
    let query = "rUsT";
    let contents = r#"
Rust:
safe, fast, productive.
Pick three.
Trust me."#;

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}

/// -- 测试配置构建成功的情况
#[test]
fn test_config_build_success() {
    let args = vec![
        String::from("program"),
        String::from("query"),
        String::from("file.txt"),
    ];
    let config = Config::build(args.into_iter());
    assert!(config.is_ok());
}

/// -- 测试配置构建缺少查询参数的情况
#[test]
fn test_config_build_missing_query() {
    let args = vec![String::from("program")];
    let config = Config::build(args.into_iter());
    assert!(matches!(
        config.unwrap_err(),
        minigrep::ConfigError::MissingQuery
    ));
}

/// -- 测试配置构建缺少文件路径的情况
#[test]
fn test_config_build_missing_file_path() {
    let args = vec![String::from("program"), String::from("query")];
    let config = Config::build(args.into_iter());
    assert!(matches!(
        config.unwrap_err(),
        minigrep::ConfigError::MissingFilePath
    ));
}
