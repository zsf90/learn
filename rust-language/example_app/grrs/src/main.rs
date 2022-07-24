use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
enum Sex {
    Man(String),
    Female(String),
}

#[derive(Debug)]
struct Student {
    name: String,
    sex: Sex,
    age: u16,
    id: u64,
}

impl Student {
    fn new(name: &str, sex: Sex, age: u16, id: u64) -> Student {
        Student {
            name: name.to_string(),
            sex,
            age,
            id,
        }
    }
}

trait Hello {
    fn say_hi(&self) {
        println!("hi");
    }
}

impl Hello for Student {
    // 重写 say_hi()
    fn say_hi(&self) {
        println!("name: {},  age: {}, id: {}", self.name, self.age, self.id);
    }
}

fn main() {
    /*
    let zhangsan = Student::new("张三", Sex::Man(String::from("男")), 24, 1);

    println!("{:#?}", zhangsan);

    zhangsan.say_hi();
    */
}
