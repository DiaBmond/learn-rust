mod box_examples;
mod deref_examples;
mod drop_examples;
mod rc_examples;
mod refcell_examples;

fn main() {
    box_examples::run();
    deref_examples::run();
    drop_examples::run();
    rc_examples::run();
    refcell_examples::run();
}
