use std::error::Error;
use std::fmt;
use std::{env, fs};

/// 自定义错误类型，用于处理配置相关的错误
#[derive(Debug)]
pub enum ConfigError {
    MissingQuery,
    MissingFilePath,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingQuery => write!(f, "Didn't get a query string"),
            ConfigError::MissingFilePath => write!(f, "Didn't get a file path"),
        }
    }
}

impl Error for ConfigError {}

/// 定义配置结构体，用于存储搜索参数
#[derive(Debug)]
pub struct Config {
    query: String,     // -- 查询字符串
    file_path: String, // -- 文件路径
    ignore_case: bool, // -- 是否忽略大小写
}

impl Config {
    /// 从命令行参数构建配置实例
    ///
    /// # Arguments
    ///
    /// * `args` - 命令行参数迭代器
    ///
    /// # Returns
    ///
    /// * `Result<Config, ConfigError>` - 成功返回配置实例，失败返回错误
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        // -- 跳过程序名称参数
        args.next();

        // -- 获取查询字符串
        let query = args.next().ok_or(ConfigError::MissingQuery)?;

        // -- 获取文件路径
        let file_path = args.next().ok_or(ConfigError::MissingFilePath)?;

        // -- 检查环境变量以决定是否忽略大小写
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // -- Getter 方法
    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

/// 运行搜索功能
///
/// # Arguments
///
/// * `config` - 搜索配置
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - 成功返回 Ok(()), 失败返回错误
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;

    let results = if config.ignore_case() {
        search_case_insensitive(config.query(), &contents)
    } else {
        search(config.query(), &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// 执行大小写敏感搜索
///
/// # Arguments
///
/// * `query` - 搜索关键词
/// * `contents` - 要搜索的文本内容
///
/// # Returns
///
/// * `Vec<&str>` - 匹配的行列表
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 执行大小写不敏感搜索
///
/// # Arguments
///
/// * `query` - 搜索关键词
/// * `contents` - 要搜索的文本内容
///
/// # Returns
///
/// * `Vec<&str>` - 匹配的行列表
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
