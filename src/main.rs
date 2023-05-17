use std::env;
use std::process;

use mingrep::Config;

fn main() {
    //读取命令行参数
    let args: Vec<String> = env::args().collect(); 

    //尝试解析命令行参数，并将其转换为一个 Config 类型的对象
    //如果解析失败，则打印错误信息并退出程序
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = mingrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
