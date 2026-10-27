use traits::{NewsArticle, SocialPost, Summary};
use traits::{
    notify_both, notify_use_generic_one_para, notify_use_generic_two_para,
    notify_use_impl_one_para, notify_use_impl_two_para,
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
}
