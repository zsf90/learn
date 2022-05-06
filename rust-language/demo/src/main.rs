#![allow(dead_code)]
/// # 该项目为测试项目
/// `日期：2022-4-30`
///
///
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32, // Trait
{
    /// 参数为Fn 类型的闭包，返回一个 Cacher 结构体
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    /// 设置或获取值
    /// 如果 Cacher.value 为空则会调用闭包 self.calculation
    /// 如果 Cacher.value 不为空，则会这行其他操作
    /// 当执行 value() 函数时会执行闭包或返回 value 的值
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if v == arg {
                    v
                } else {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // generate_workout(22, 7);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    println!("{}", expensive_result.value(13));
    println!("{}", expensive_result.value(13));
    println!("{}", expensive_result.value(13));
    println!("{}", expensive_result.value(13));
    println!("{}", expensive_result.value(3));
    println!("{}", expensive_result.value(3));
    println!("{}", expensive_result.value(3));

    let mut cacher = Cacher::new(|num| {
        println!("cacher out!");
        thread::sleep(Duration::from_secs(1));
        num
    });
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));
    println!("{}", cacher.value(10));
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));

//     intensity
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     // let expensive_result = simulated_expensive_calculation(intensity);

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
