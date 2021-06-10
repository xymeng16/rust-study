#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // print something
        println!("{:?}", self);
    }
}

// Now, let's talk about Option<T>
// The definition of Option<T> in the standard library is:
// enum Option<T> {
//     Some(T),
//     None,
// }
// It is such a very useful feature in Rust, hence included
// in prelude. You can use it without Option::


fn main() {
    let message = Message::Move { x: 1, y: 2 };
    message.call();

    let _some_number = Some(1);
    let _some_string = Some(String::from("Hello"));
    let _absent_number: Option<String> = None; // use None need to explicitly indicate the type


    let x:i8 = 5;
    let y:Option<i8> = Some(5);

    // let sum = x + y; // not work, you cannot add i8 to Option<i8>
    let _sum = x + y.expect("something wrong happened");
}
