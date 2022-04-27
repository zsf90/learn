// Rust Result 例子
use std::io;

fn main() {
    println!("Hello, world!");
    let n1 = 0b0010_0110;
    println!("N1: {}", n1);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
    tup.1 = 200.234;
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("X: {}, Y: {}, Z: {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    println!("A: {}", a[0]);

    let b = ["Apple", "Banana", "Tangerine"];
    println!("B[0]: {}, B[1]: {}, B[2]: {}", b[0], b[1], b[2]);

    let mut c: [i16; 5] = [9; 5];
    c[0] = 2;
    println!(
        "C[0]: {}, C[1]: {}, C[2]: {}, C[3]: {}, C[4]: {}",
        c[0], c[1], c[2], c[3], c[4]
    );

    println!("请选择数组a中的元素，0~4");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("请输入有效的索引值");

    let index: usize = index
        .trim()
        .parse()
        .expect("请输入整数数字，不能输入非数字类型");

    let element = a[index];
    println!("您选择的索引[{}] = {}", index, element);

    let y = {
        let x = 3;
        x + 1
    };
    println!("Y: {}", y);

    if y == 0 {
        println!("hello");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    for number in (0..101).rev() {
        println!("{}", number);
    }
    let p = {
        let p = 22;
        p
    };
    println!("P: {}", p);

    let mut name = String::from("张山峰");
    // name = name + " 你好！";
    name.push_str(" 你好!");
    println!("Name: {}", name);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
