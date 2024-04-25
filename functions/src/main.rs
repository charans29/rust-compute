fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    let y = {
       let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");

    let five = five();

    another_fn(five, 'k');

    let x = plus_one(10);

    println!("The value of x is: {x}");

    println!("expression: {}", f({
        let y = 1;
        y + 1
      }));
}

fn f(x: i32) -> i32 { x + 1 }

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn another_fn(x: i32, bit: char){
    println!("Output from another_fn: {x}{bit}");
}

