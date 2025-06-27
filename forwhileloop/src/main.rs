fn main() {

    for i in 0..3 { // for <loop valuable> in start..end 
        println!("{}", i)
    }

    for _ in 1..=3 { // include 5
        println!("check")
    }

    let (start, end) = (3, 5);
    for x in start..=end {
        println!("#{x}");
    }

    for i in -3..=0 {
        println!("{i}");
    }
}
