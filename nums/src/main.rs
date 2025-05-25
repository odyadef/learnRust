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

    let pi = 3.1415_f64;

    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let truncated = x / y;
    let remainder = x % y;
    let xpi = x as f64 * pi;

    println!("sum = {sum}\ndifference = {difference}\nproduct = {product}\ntruncated = {truncated}\nremainder = {remainder}");
    println!("{x} * {pi} = {xpi:.2}");

    println!("{} = {} - {}",x, y, x == y);

}
