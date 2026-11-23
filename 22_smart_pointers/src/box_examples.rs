enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn run() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    print_list(&list);
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            println!("list item = {value}");
            print_list(next);
        }
        List::Nil => {}
    }
}
