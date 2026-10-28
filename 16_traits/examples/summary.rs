use traits::{Describe, NewsArticle, Pair, SocialPost, Summary};
use traits::{
    notify_both, notify_both_where, notify_use_generic_one_para, notify_use_generic_two_para,
    notify_use_impl_one_para, notify_use_impl_two_para, returns_summarizable,
    returns_summarizable_with_switch,
};

fn main() {
    // 1. Required and default trait methods

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // Both types use the default summarize() method.
    println!("1 new post: {}", post.summarize());
    println!("New article available! {}", article.summarize());

    // 2. Traits as function parameters

    notify_use_impl_one_para(&article);
    notify_use_generic_one_para(&post);

    // impl Trait allows different concrete types for each parameter.
    notify_use_impl_two_para(&post, &article);

    // A shared T requires both arguments to have the same concrete type.
    notify_use_generic_two_para(&post, &post);

    // Won't compile: SocialPost and NewsArticle are different types.
    // notify_use_generic_two_para(&post, &article);

    // 3. Multiple trait bounds and where clauses

    // String implements Display and our custom Summary trait.
    notify_both(&String::from("user01"));
    notify_both_where(&String::from("mon"));

    // 4. Returning types that implement traits

    let returned_post = returns_summarizable();
    println!("{}", returned_post.summarize());

    let switched_post = returns_summarizable_with_switch(true);
    println!("{}", switched_post.summarize());

    // 5. Conditionally implementing methods

    // i32 implements both Display and PartialOrd.
    let numbers = Pair::new(10, 20);
    numbers.cmp_display();

    // String also implements both traits.
    let words = Pair::new(String::from("apple"), String::from("banana"));
    words.cmp_display();

    // Vec<i32> can be used with new(), but not cmp_display().
    let _lists = Pair::new(vec![1, 2], vec![3, 4]);

    // Won't compile: Vec<i32> doesn't implement Display.
    // _lists.cmp_display();

    // 6. Blanket implementations

    // Standard library: Display types also implement ToString.
    let number = 42;
    let text = number.to_string();
    println!("{text}");

    // Our blanket implementation: every Display type gets Describe.
    println!("{}", 42.describe());
    println!("{}", "hello".describe());
    println!("{}", 3.5.describe());

    // Vec<i32> doesn't implement Display, so it doesn't get Describe.
    let _numbers = Vec::from([1, 2, 3]);

    // Won't compile: Vec<i32> doesn't implement Display.
    // _numbers.describe();
}
