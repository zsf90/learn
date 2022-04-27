#![allow(unused_imports)]
extern crate trait_lib_demo;

use trait_lib_demo::{zsf_tt, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let new_article = NewsArticle {
        headline: String::from("Ubuntu22.04今天发布了"),
        location: String::from("China"),
        author: String::from("zsf"),
        content: String::from("ubuntu22 今天发布了，有时间一定要试试。"),
    };

    println!("1 new New Article: {}", new_article.summarize());

    zsf_tt::notify(&tweet, &new_article);
}
