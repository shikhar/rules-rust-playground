use value_bag::ValueBag;

fn main() {
    let bag = ValueBag::capture_display(&42);
    let num = bag.to_u64().unwrap();
    println!("hello {}", num);
}
