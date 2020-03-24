use std::mem::size_of;
use std::mem::size_of_val;
struct Pt{
    x: u32,
    y: u32,
}

enum EmptyEnum{
    One,
    Two,
}
enum HoldingOneByteEnum{
    Black,
    White(u8),
}

enum HoldingStringEnum{
    One,
    Two(String),
}

enum HoldingSliceEnum<'a>{
    One,
    Two(&'a str),
}

fn main() {
    println!("Hello, world!");
    println!("sizeof struct pt is:{}",size_of::<Pt>());
    println!("sizeof struct String is:{}",size_of::<String>());
    // println!("sizeof struct str is:{}",size_of::<str>());
    println!("sizeof struct &str is:{}",size_of::<&str>());
    println!("size of enum EmptyEnum is:{}",size_of::<EmptyEnum>());
    println!("size of enum HoldingOneByteEnum is:{}", size_of::<HoldingOneByteEnum>());
    println!("size of enum HoldingStringEnum is:{}", size_of::<HoldingStringEnum>());
}
