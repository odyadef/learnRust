use std::io;

fn main() {
    println!("Please, tape your name: ");
    let mut name = String::new();
    let res = io::stdin()
        .read_line(&mut name)
        .expect("Cannot read input");
    println!("Hello, {}!", name.trim());
    println!("result = {}", res);

    println!("Please, tape first number");
    let mut first_num = String::new();
    io::stdin()
        .read_line(&mut first_num)
        .expect("Cannot read input");
    let x: i32 = first_num.trim().parse().expect("Please, tape a number");
    
    println!("Please, tape second number");
    let mut second_num = String::new();
    io::stdin()
        .read_line(&mut second_num)
        .expect("Cannot read input");
    let y: i32 = second_num.trim().parse().expect("Please, tape a number");    
    
    println!("{} + {1} = {sum}", x, y, sum = x + y);
}
