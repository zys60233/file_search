//官方库
use std::env;
use std::process;

//项目库
use file_search::Config;


fn main() {
    //环境参数
    let args: Vec<String> = env::args().collect();

    //配置变量
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);

    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.file_path);

    if let Err(e) = file_search::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}