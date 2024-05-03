fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    // let a_num = 4;
    make_and_drop();
}

fn make_and_drop() {
    let a_box = Box::new(5);
    let _b = a_box;
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
