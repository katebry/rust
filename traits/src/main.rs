pub trait Summary {
    fn summarise_author(&self) -> String;

    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaing news! {}", item.summarise());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarise());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarise());
}
