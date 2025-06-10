use std::io;

fn main() {
    println!("Type a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read input!");
    let num:i32 = input.trim().parse().expect("Must be a number");
    let res = num > 0;
    if res {
        println!("{num} is positive");
        if num % 2 == 0 {
            println!("and {num} is even");
        } else {
            println!("and {num} is odd");
        }
    } else if num == 0 {
        println!("{num} = 0");
    } else {
        println!("{num} is negative");
    }

    let mark = if res {"!"} else {"?"};
    println!("{mark}");

    let abs = if res {
        num
    } else {
        num * (-1)
    };
    println!("|{num}| = {abs}");

    println!("Say hello!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read input!");
    let hello = input.trim();
    if hello != "hello" && hello != "Hello" {
        println!("Please, say hello, not {hello}!");
    } else {
        println!("Thank you!");
    }
}

/*
&& - AND
|| - OR
! - NOT
*/
