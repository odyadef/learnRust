fn main() {
    let arr = [1, 2, 3];
    let arr1 = [11];
    let arr2 = [12,];

    let arr3: [u8; 3] = [22, 23, 24];
    let arr4 = [33_i16, 34, 35];

    let zero_arr = [0; 6];

    println!{"{:?}", arr};
    println!{"{:?}", arr1};
    println!{"{:?}", arr2};
    println!{"{:?}", arr3};
    println!{"{:?}", arr4};
    println!{"{zero_arr:?}"};

    let emoji_arr: [char; 6];
    emoji_arr = ['🦀', '🐈', '🐦', '🐓', '🦫', '🦄'];

    println!("{emoji_arr:?}");
    println!("{}", emoji_arr[5]);

    let (first, second, third) = (0, 1, 2);
    println!("{}, {}, {}", emoji_arr[first], emoji_arr[second], emoji_arr[third]);

    let mut c_arr = ['a', 'b'];
    println!("{c_arr:?}");
    c_arr[0] = emoji_arr[4];
    println!("{c_arr:?}");

    let [cr, ct, bd, rr, bv, un] = emoji_arr;
    println!("crab {cr}\ncat {ct}\nbird {bd}\nrooster {rr}\nbeaver {bv}\nunicorn {un}")

}
