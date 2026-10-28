use std::fmt::Display;

// 1. Traits: required methods and default methods

pub trait Summary {
    // Required method: each implementing type must define this.
    fn summarize_author(&self) -> String;

    // Default method: implementing types can use or override this.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Uncomment to override the default summarize() method.
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

// String already implements Display.
// Adding Summary lets String satisfy both trait bounds.
impl Summary for String {
    fn summarize_author(&self) -> String {
        self.clone()
    }
}

// 2. Using traits as function parameters

// impl Trait syntax: accepts any type implementing Summary.
pub fn notify_use_impl_one_para(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Generic trait bound syntax: equivalent for one parameter.
pub fn notify_use_generic_one_para<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Each impl Summary parameter can have a different concrete type.
pub fn notify_use_impl_two_para(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Both parameters must have the same concrete type T.
pub fn notify_use_generic_two_para<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// 3. Multiple trait bounds and where clauses

// T must implement both Summary and Display.
pub fn notify_both<T: Summary + Display>(item: &T) {
    println!("Display: {item}");
    println!("Summary: {}", item.summarize());
}

// Same bounds as notify_both(), written with a where clause.
pub fn notify_both_where<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Display: {item}");
    println!("Summary: {}", item.summarize());
}

// 4. Returning types that implement traits

// The return type is hidden, but the concrete type is SocialPost.
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("Learning Rust traits"),
        reply: false,
        repost: false,
    }
}

// Compiles: both branches return the same concrete type.
pub fn returns_summarizable_with_switch(switch: bool) -> impl Summary {
    if switch {
        SocialPost {
            username: String::from("mon"),
            content: String::from("Hello"),
            reply: false,
            repost: false,
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("Learning Rust"),
            reply: false,
            repost: false,
        }
    }
}

// Won't compile: impl Summary requires one concrete return type.
// NewsArticle and SocialPost are different types, even though both
// implement Summary.
//
// fn returns_mixed_types(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::new(),
//             location: String::new(),
//             author: String::new(),
//             content: String::new(),
//         }
//     } else {
//         SocialPost {
//             username: String::new(),
//             content: String::new(),
//             reply: false,
//             repost: false,
//         }
//     }
// }

// 5. Conditionally implementing methods

pub struct Pair<T> {
    x: T,
    y: T,
}

// Available for every Pair<T>.
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Available only when T implements both Display and PartialOrd.
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 6. Blanket implementations

pub trait Describe {
    fn describe(&self) -> String;
}

// Implement Describe for every type T that implements Display.
impl<T: Display> Describe for T {
    fn describe(&self) -> String {
        format!("Value: {self}")
    }
}
