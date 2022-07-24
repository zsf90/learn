fn main() {
    fahrenheit_to_celsius();
}

fn fahrenheit_to_celsius() {
    let mut celsius = String::new(); // 摄氏度
    let mut fahrenheit = String::new(); // 华氏度
    let mut to_type = String::new(); // 为 0 摄氏度转华氏度，为 1 华氏度转摄氏度

    println!("请选择转换类型(输入 '0' 代表摄氏温度转华氏温度，输入'1' 代表华氏温度转摄氏温度)");
    // 1. 让用户选择转换类型
    std::io::stdin()
        .read_line(&mut to_type)
        .expect("读取数据失败");

    let to_type = match to_type.trim().parse() {
        Ok(v) => v,
        Err(e) => panic!("数据输入类型错误：{}", e),
    };
    let result = match to_type {
        0 => {
            println!("请输入摄氏温度");
            std::io::stdin()
                .read_line(&mut celsius)
                .expect("数据读取失败");

            let celsius: f64 = match celsius.trim().parse() {
                Ok(v) => v,
                Err(e) => panic!("数据输入类型错误：{}", e),
            };
            Some(9.0 / 5.0 * celsius + 32.0)
        }
        1 => {
            // C = 5 / 9 (F-32)
            println!("请输入华氏温度");
            std::io::stdin()
                .read_line(&mut fahrenheit)
                .expect("数据读取失败");
            let fahrenheit: f64 = match fahrenheit.trim().parse() {
                Ok(v) => v,
                Err(e) => panic!("数据输入类型错误：{}", e),
            };
            Some(5.0 / 9.0 * (fahrenheit - 32.0))
        }
        _ => None,
    };

    match result {
        Some(v) => {
            println!("结果为：{}", v);
        }
        None => println!("选择错误!,只能输入 0 或 1"),
    }
}
