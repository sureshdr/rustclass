use uuid::Uuid;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let curr_id = Uuid::new_v4();
    println!("I have generated uuid v4:  {}",
        curr_id);
    print_type_of(&curr_id)
}
