use std::io;

fn main() {
    println!("Please, tape your name: ");
    let mut name = String::new();
    let res = io::stdin()
        .read_line(&mut name)
        .expect("Cannot read string");
    println!("Hello, {name}!");
    println!("result = {}", res);
    let x = 10;
    let y = 15;
    println!("{} + {1} = {sum}", x, y, sum = x + y);
}
