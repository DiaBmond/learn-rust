mod closures;
mod inventory;
mod iterators;
mod rectangle;

fn main() {
    inventory::run_demo();

    closures::closure_type_inference_demo();
    closures::immutable_capture_demo();
    closures::mutable_capture_demo();
    closures::move_capture_demo();

    rectangle::sort_by_key_demo();
    iterators::run_demo();
}
