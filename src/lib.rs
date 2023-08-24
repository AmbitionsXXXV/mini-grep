use std::error::Error;
use std::{env, fs};

// 定义配置结构体
pub struct Config {
    pub query: String,     // 查询字符串
    pub file_path: String, // 文件路径
    pub ignore_case: bool, // 是否忽略大小写
}

impl Config {
    // 构建配置结构体的实例
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments"); // 参数不足，返回错误
        // }
        args.next();

        // 存储命令行参数
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 检查环境变量是否存在，以决定是否忽略大小写
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // 返回构建好的配置实例
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// 运行搜索功能
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容到字符串
    let contents = fs::read_to_string(config.file_path)?;

    // 根据是否忽略大小写，调用不同的搜索函数
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // 打印搜索结果
    for line in results {
        println!("{line}");
    }

    Ok(())
}

// 执行大小写敏感搜索
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    //
    // // 遍历每一行内容，检查是否包含查询字符串
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    // 引入迭代器后
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 执行大小写不敏感搜索
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // 将查询字符串转为小写

    // let mut results = Vec::new();
    //
    // // 遍历每一行内容，检查是否包含小写的查询字符串
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    //
    // results

    // 引入迭代器后
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
