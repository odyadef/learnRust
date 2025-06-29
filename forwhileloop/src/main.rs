use std::io;

fn main() {

    for i in 0..3 { // for <loop valuable> in start..end 
        println!("{}", i)
    }

    for _ in 1..=3 { // include 3
        println!("check")
    }

    let (start, end) = (3, 5);
    for x in start..=end {
        println!("#{x}");
    }

    for i in -3..=0 {
        println!("{i}");
    }

    let mut input = String::new();
    for _ in 0..3 {
        println!("Please type a number");
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input");
        let num: i32 = input.trim()
            .parse().expect("Must be a number");
        println!("Number #{num}");

        input.clear();
    }
}
