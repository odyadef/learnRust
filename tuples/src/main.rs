fn main() {
    let tup = ('🦀', 666, 7.2, "Rust");
    let a_tup: (i8, char, f32) = (8, '🐈', 2.5);
    let s_tup = (1312_i32, 1.1_f32, '🐦');
    println!("{} {} {}", tup.0, a_tup.1, s_tup.2);

    let mut e_tup: (char, char, char) = ('a', 'b', 'c');
    println!("{} {} {}", e_tup.0, e_tup.1, e_tup.2);
    e_tup.0 = '🐓';
    e_tup.1 = '🦫';
    e_tup.2 = '🦄';
    println!("{:?}", e_tup);
}
