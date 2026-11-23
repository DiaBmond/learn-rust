use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn run() {
    let x = 5;
    let reference = &x;

    assert_eq!(5, x);
    assert_eq!(5, *reference);

    let boxed = Box::new(x);
    assert_eq!(5, *boxed);

    let custom = MyBox::new(x);
    assert_eq!(5, *custom);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
