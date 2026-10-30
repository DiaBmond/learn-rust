// A struct that holds a reference must declare a lifetime parameter.
// The referenced data must remain valid while the field is used.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Returns an i32 rather than a reference.
    // No output lifetime annotation is needed.
    fn level(&self) -> i32 {
        3
    }

    // Lifetime Elision Rule 3:
    // The returned reference is tied to the borrow of self,
    // not to the lifetime of announcement.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    // first_sentence is a string slice borrowed from novel.
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Part: {}", excerpt.part);
    println!("Level: {}", excerpt.level());

    let result;

    {
        let announcement = String::from("New excerpt");

        result = excerpt.announce_and_return_part(&announcement);
    } // announcement is dropped here.

    // This compiles because the returned reference is tied to
    // the borrow of excerpt, not to announcement.
    // Both excerpt and the referenced data in novel are still valid.
    println!("Returned part: {result}");
}
