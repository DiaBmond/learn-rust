use traits::Describe;
use traits::{NewsArticle, Pair, SocialPost, Summary};
use traits::{
    notify_both, notify_both_where, notify_use_generic_one_para, notify_use_generic_two_para,
    notify_use_impl_one_para, notify_use_impl_two_para, returns_summarizable,
    returns_summarizable_with_switch,
};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

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

    notify_use_generic_one_para(&post);
    notify_use_impl_one_para(&article);

    notify_use_impl_two_para(&post, &article);
    // notify_use_generic_two_para(&post, &article); // not compile error  use impl is para do have para must same type
    notify_use_generic_two_para(&post, &post);
    notify_both(&String::from("user01"));
    notify_both_where(&String::from("mon"));

    let result = returns_summarizable();

    println!("{}", result.summarize());

    let post = returns_summarizable_with_switch(true);
    println!("{}", post.summarize());

    let numbers = Pair::new(10, 20);
    numbers.cmp_display();

    let _lists = Pair::new(vec![1, 2], vec![3, 4]);

    // lists.cmp_display(); // Won't compile: Vec<i32> doesn't implement Display.

    let words = Pair::new(String::from("apple"), String::from("banana"));
    words.cmp_display();

    let number = 42;
    let text = number.to_string();

    println!("{text}");

    println!("{}", 42.describe());
    println!("{}", "hello".describe());
    println!("{}", 3.5.describe());

    let _numbers = vec![1, 2, 3];

    // numbers.describe();
}
