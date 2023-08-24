// 引入 minigrep 模块中的 run 和 Config 结构体
use minigrep::{run, Config};
// 引入标准库中的 env 和 process 模块
use std::{env, process};

fn main() {
    // 获取命令行参数，并将它们存储在一个字符串向量中
    let args: Vec<String> = env::args().collect();

    // 尝试构建 Config 结构体，如果出现错误则进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err); // 打印错误信息
        process::exit(1); // 退出程序，返回错误状态码
    });

    // 打印要搜索的目标和文件路径
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 尝试运行 run 函数，处理可能的错误
    if let Err(e) = run(config) {
        // println!("Application error: {}", e); // 打印错误信息
        // 使用 eprintln! 宏打印错误信息
        eprintln!("Application error: {e}");
        process::exit(1); // 退出程序，返回错误状态码
    }
}
