use std::mem::size_of;
use std::mem::size_of_val;

fn main() {
    // enum Message {
    //     Quit,
    //     Move ,
    //     Write,
    //     ChangeColor,
    // };
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    };
    
    struct Point{x:i32,y:i32};
    println!("size of Message is {}",size_of::<Message>());
    println!("size of Message.Quit is {}",size_of_val(&Message::Quit));
    println!("size of Message.Move is {}",size_of_val(&Message::Move{x:0,y:0}));
    println!("size of Message.Write(String) is {}",size_of_val(&Message::Write(String::from("a string"))));
    println!("size of Point is {}",size_of::<Point>());
    println!("size of &String is {}",size_of::<&String>());
    println!("size of String is {}",size_of::<String>());
    println!("size of (String) is {}",size_of::<(String)>());
    println!("size of (i32,i32,i32) is {}",size_of::<(i32,i32,i32)>());

}
