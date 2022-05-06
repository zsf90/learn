今天就来学习记录下`Rust`中的必掌握知识之 `Trait`。

`Trait` 是`Rust` 中的接口，可以为不同的类型提供同一类行为（函数）。

## 定义一个 Trait

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}
```

上面代码定义了一个名字为 `Summary`的 `Trait` ，并且里面分别声明了一个名为 summarize_author() 的函数和一个函数定义 `summarize()`。

## 定义两个结构体并且实现 Summary Trait

```rust
//  NewsArticle 结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 对 NewsArticle 实现 Summary
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

// Tweet 结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// 对 Tweet 实现 Summary Trait
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```