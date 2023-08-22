use std::{env, fs, process};

fn main() {
    // 接受命令行参数
    let args: Vec<String> = env::args().collect();

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 文件读取
    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        // 存储命令行参数
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn parse_config(args: &[String]) -> Config {
    // 存储命令行参数
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
