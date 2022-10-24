// Trait
// pub trait Summary {
//     fn summarize(&self) -> String;
// }


pub trait Summary {
    fn summarize_author(&self) -> String;

    // Trait with default behavior
    fn summarize(&self) -> String {
        String::from("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Use the default trait behavior
// impl Summary for NewsArticle {}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement the trait without a default behavior, allow default for the other function
impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// accepts any type that implements Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// returns any type that implements Summary
fn returns_summarizable() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Implement the trait for the struct
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }


/* assuming this was a crate called aggregator this is how a binary could use it and the traits
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

 */

/* This is how we'd use the default behavior for NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
 */

/*  This is how we'd use the trait that has two functions with one default behavior
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
 */

/* Begin default boilerplate */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
