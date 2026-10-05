fn main() {
    first();
}

fn first() {
    second();
}

fn second() {
    third();
}

fn third() {
    panic!("Something went wrong");
}
