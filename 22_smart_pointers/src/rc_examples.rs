use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};

pub fn run() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    assert_eq!(18, sum(&b));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        assert_eq!(19, sum(&c));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, next) => value + sum(next),
        Nil => 0,
    }
}
