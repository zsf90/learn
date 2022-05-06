#![allow(unused_variables)]
#![allow(dead_code)]
/// # Rust 命令行程序初试
use command_demo::{self, Config};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // 收集命令行参数

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = command_demo::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
