fn main() {
    // let x = sum(3,4);
    // println!("hello, {x} !")
    mutability()
}

fn mutability() {
    // let x = 5;
    let mut x = 5;
    println!("Hello, {x}!");
    x = 6;
    println!("Hello, {x}!");
}

fn shadowing() {
    let x = 5;
    println!("Hello, {x}!");
    let x = 6;
    println!("Hello, {x}!");

    {
        let x = x * 2;
        println!("Hello, inner scope's {x}!");
    }

    println!("Hello, outer scope's {x}!");
}

fn sum(x :i32, y :i32) -> i32 {
    x + y
}
