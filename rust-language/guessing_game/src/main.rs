// Rust 猜字游戏

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("=== 猜字游戏开始 ===");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // Loop 循环
    loop {
        print!("请输入一个0~100之间的整数: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new(); /* 创建一个可变字符串变量  */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("~~~~您输入的数字偏小了~~~~"),
            Ordering::Greater => println!("~~~~您输入的数字偏大了~~~~~"),
            Ordering::Equal => {
                println!("^_^恭喜您猜对了^_^");
                break;
            }
        }
    } // Loop 循环结束
}
