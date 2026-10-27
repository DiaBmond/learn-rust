// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

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

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// String already implements Display.
// Implement Summary for String to satisfy both trait bounds.
impl Summary for String {
    fn summarize_author(&self) -> String {
        self.clone()
    }
}

pub fn notify_use_impl_one_para(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_use_generic_one_para<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_use_generic_two_para<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify_use_impl_two_para(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify_both<T: Summary + std::fmt::Display>(item: &T) {
    println!("Display: {item}");
    println!("Summary: {}", item.summarize());
}

pub fn notify_both_where<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("Display: {item}");
    println!("Summary: {}", item.summarize());
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("Learning Rust traits"),
        reply: false,
        repost: false,
    }
}

// Won't compile: `impl Summary` requires the same concrete
// return type in every branch.
// fn returns_summarizable_with_switch(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             repost: false,
//         }
//     }
// }

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
