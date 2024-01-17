pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("Default Traits")
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("@John Doe"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        author: String::from("John k Doe"),
        headline: String::from("Sky is falling"),
        content: String::from("Lies!!!"),
    };
    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());
    notify(&article);
    notify(&tweet);
}
