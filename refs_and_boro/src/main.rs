fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // greet(m1, m2);
    let (m1_again, m2_again) = greet(&m1, &m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("{}",s);

    test();

    _explicit_and_implicit();

    exam();

    let mut q: i32 = 10;
    _lifetime(&mut q);

    return_a_string();

    let mut v = vec![1, 2, 3];
    let n = v[0];
    // give_and_take(&v, 4);
    let k = give_and_take(&mut v, 4);
    println!("{} {}", n, k);

}

fn give_and_take(v: &mut Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0);
    n
}

fn greet(g1: &String, g2: &String) -> (String, String) {
    (g1.to_string(), g2.to_string())
}

fn test(){
    let mut x: Box<i32>  =  Box::new(10);
    let a: i32 = *x;                        // a = 10 reads from stack
    *x += 10;                               // x = 20  changes in heap
    println!("{} {}",a, *x);

    let r1: &Box<i32> = &x;                 // r1 = 20 points to stack
    let b: i32 = **r1;                      // b = 20 gets from heap
    println!("{} {}", b, r1);

    let r2: &i32 = &*x;                     // r2 = 20 points to heap
    let c: i32 = *r2;                       // c = 20 reads from satck
    println!("{} {}", c, r2);
}

fn _explicit_and_implicit(){
    let x: Box<i32> = Box::new(-10);
    let y: Box<i32> = Box::new(10);
    let x_abs1= i32::abs(*x);           // Explicit deref by *
    let x_abs2 = y.abs();               // implicit deref by '.'
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r1 = i32::abs(**r);       // explicit deref twice by '**'
    let r2 = r.abs();                   // implicit deref twice by '.'
    assert_eq!(r1, r2);

    let s = String::from("hello");
    let s_len1 = str::len(&s);  // explicit refe
    let s_len2  = s.len();            // implicit refe
    assert_eq!(s_len1, s_len2);
}

fn exam(){
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 2;
    println!("Third element is {}", *num);
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2); 
    v.push(4);
    println!("Vector is now {:?}", v);
}


fn _lifetime(x: &mut i32){
    let y = &*x;        // y life starts
    let z = *y;         // y life ends
    *x += z;
    println!("{} {}", *x, z)
}

fn return_a_string() -> String {
    let s = String::from("Hello world");
    let s_ref = s;
    s_ref
}