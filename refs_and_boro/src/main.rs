fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // greet(m1, m2);
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("{}",s);
}

fn greet(g1: String, g2: String) -> (String, String) {
    (g1, g2)
}