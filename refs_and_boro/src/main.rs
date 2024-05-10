fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // greet(m1, m2);
    let (m1_again, m2_again) = greet(&m1, &m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("{}",s);

    test()
}

fn greet(g1: &String, g2: &String) -> (String, String) {
    (g1.to_string(), g2.to_string())
}

fn test(){
    let mut x: Box<i32>  =  Box::new(10);
    let a: i32 = *x;                        // a = 10 get
    *x += 10;                               // x = 20 heap
    println!("{} {}",a, *x);

    let r1: &Box<i32> = &x;                 // r1 = 20 stac
    let b: i32 = **r1;                      // b = 20 get
    println!("{} {}", b, r1);

    let r2: &i32 = &*x;                     // r2 = 20 heap
    let c: i32 = *r2;                       // c = 20 get
    println!("{} {}", c, r2);
}