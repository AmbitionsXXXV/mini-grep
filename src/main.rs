use std::{env, fs};

fn main() {
    // 接受命令行参数
    let args: Vec<String> = env::args().collect();

    // 存储命令行参数
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // 文件读取
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("With text:\n{contents}");
}
