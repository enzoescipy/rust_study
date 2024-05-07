fn main() {
    mutable_and_immutable_reference();
}

fn move_heap() {
    let str = String::from("hello, world!");
    println!("{str}");
    println!("{str}");

    let taker = str;
    println!("{taker}");
    // println!("{str}");  // panic code. tried to print the empty variable
}

fn copy_stack() {
    let number = 5;
    println!("{number}");
    println!("{number}");

    let taker = number;
    println!("{taker}");
    println!("{number}");  
}

fn taker(s:String) {
    println!("{s} been taken!!");
}

fn move_by_function() {
    let str = String::from("hello, world!");
    taker(str);
    // println!("{str}"); // panic code. tried to print the empty variable
}

fn reference_taker(s: &String) {
    println!("{s} reference been taken!!");
}

fn reference() {
    let str = String::from("hello, world!");
    reference_taker(&str);
}

fn mutable_and_immutable_reference() {
    let mut str = String::from("hello, world!");

    let str_ref_0 = &str;
    let str_ref_1 = &str;
    println!("{},{}", str_ref_0, str_ref_1);
    // str_ref_0, str_ref_1 dropped at this point

    let str_ref_mut = &mut str;
    str_ref_mut.push_str(" new_string");
    println!("{str_ref_mut}")
}