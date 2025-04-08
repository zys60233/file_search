use std::env;
use std::fs;

fn main() {
    //环境参数
    let args: Vec<String> = env::args().collect();

    //要查询的字符串
    let query = &args[1];

    //文件名
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    //文件内容
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}
