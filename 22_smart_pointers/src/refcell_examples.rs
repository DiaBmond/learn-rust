use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};

pub fn run() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    assert_eq!(15, sum(&a));
    assert_eq!(18, sum(&b));
    assert_eq!(19, sum(&c));
}

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, next) => *value.borrow() + sum(next),
        Nil => 0,
    }
}
