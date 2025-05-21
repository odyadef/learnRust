use std::io;

fn main() {
    println!("Type first number");
    let mut first = String::new();

    println!("Type second number");
    let mut second = String::new();
    
    io::stdin()
        .read_line(&mut first)
        .expect("Cannot read input");
    io::stdin()
        .read_line(&mut second)
        .expect("Cannot read input");
    
    let x: i32 = first.trim().parse().expect("Must be an integer");
    let y: i32 = second.trim().parse().expect("Must be an integer");

    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let truncated = x / y;
    let remainder = -x % y;

    println!("sum = {sum}\ndifference = {difference}\nproduct = {product}\ntruncated = {truncated}\nremainder = {remainder}");


}
