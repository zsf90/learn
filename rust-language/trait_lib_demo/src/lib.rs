///! Trait 测试
/// # 日期：2022-4-30
use std::fmt::Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// # `Trait` 定义
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/// 让 Tweet 类型实现 Display Trait
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

pub mod zsf_tt {
    use std::fmt::Display;

    use crate::Summary;

    pub fn notify<T: super::Summary, U: super::Summary>(item1: &T, item2: &U) {
        println!("Breaking news! {}", item1.summarize());
        println!("Breaking tweet! {}", item2.summarize());
    }

    /// 使用多个 Trait
    pub fn notify1<T>(item: &T)
    where
        T: super::Summary + Display,
    {
        println!("notify1: {}", item);
    }

    /// Where 语法
    pub fn print_summary<T>(item: &T)
    where
        T: super::Summary,
    {
        println!("print_summary: {}", item.summarize());
    }

    pub fn returns_summarizable() -> impl Summary {
        super::Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
