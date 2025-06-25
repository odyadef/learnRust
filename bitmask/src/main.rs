fn main() {
    let byte1 = 0x00;
    let mask1 = 0x01;

    println!("{:08b} :byte", byte1);
    println!("{:08b} :mask", mask1);
    println!("{:08b} :byte | mask", byte1 | mask1);

    println!("************************");

    let byte2 = 0xff;
    let mask2 = 0x0f;

    println!("{:08b} :byte", byte2);
    println!("{:08b} :mask", mask2);
    println!("{:08b} :byte & mask", byte2 & mask2);
}
