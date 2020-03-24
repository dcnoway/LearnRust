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
    println!("sizeof u32 is:{}",size_of::<u32>());
    println!("sizeof struct pt is:{}",size_of::<Pt>());

    // println!("sizeof struct str is:{}",size_of::<str>());

    println!("size of enum EmptyEnum is:{}",size_of::<EmptyEnum>());

    println!("size of u8 BYTE is:{}", size_of::<u8>());
    println!("size of enum HoldingOneByteEnum is:{}", size_of::<HoldingOneByteEnum>());

    println!("sizeof struct String is:{}",size_of::<String>());
    println!("size of enum HoldingStringEnum is:{}", size_of::<HoldingStringEnum>());

    println!("sizeof struct &str is:{}",size_of::<&str>());
    println!("size of enum HoldingSliceEnum is:{}", size_of::<HoldingSliceEnum>());

    let pt =Pt{x:1,y:2};
    unsafe{
        //const * rawptr = &pt;
    }
}
