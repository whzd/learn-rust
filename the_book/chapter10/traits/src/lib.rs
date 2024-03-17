use std::fmt::Display;

pub trait Summary {
    fn sumarize_author(&self) -> String;

    fn sumarize(&self) -> String {
        format!("(Read more from {}...)", self.sumarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.sumarize());
}

// To allow parameters to have different types
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.sumarize());
    println!("Breaking news! {}", item2.sumarize());
}
// To force both parameters to be the same type
pub fn notify_same_two<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.sumarize());
    println!("Breaking news! {}", item2.sumarize());
}

/*
// Multiple trait bounds
// allow different types
pub fn notify_and_display(item: &(impl Summary + Display)) {}
// force same type
pub fn notify_and_display_same<T: Summary + Display>(item: &T){}
*/

/*
// Where clauses
// base function
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// using where
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
*/

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /*
    fn sumarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    */
    fn sumarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn sumarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
