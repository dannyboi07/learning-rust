use traits::{Summary, Tweet};

fn main() {
    println!("Hello, world!");

    let tweet: Tweet = Tweet {
        username: String::from("dannyboi07"),
        content: String::from("sup brah?"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
}
