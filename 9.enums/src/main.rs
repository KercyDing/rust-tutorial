fn main() {
    // *--- 1. 使用 Message --- //
    // 创建一个实例
    let q = Message::Quit;
    let msg = Message::Move { x: 10, y: 20 };
    let w = Message::Write(String::from("Hello"));
    let color = Message::ChangeColor(1, 3, 5);
    
    // 调用它的方法
    q.call();
    msg.call(); 
    w.call();
    color.call();

    // *--- 2. 使用 MyOption --- //
    let ptr: MyOption<String> = MyOption::None;
    let s = MyOption::Some(String::from("Man!"));

    // 场景 A: 处理 None
    match ptr {
        MyOption::Some(content) => println!("ptr's content: {}", content),
        MyOption::None => println!("ptr is empty!"),
    }

    // 场景 B: 处理 Some
    match s {
        // 这里的data就是解构出来的数据
        MyOption::Some(data) => println!("s: {}", data),
        MyOption::None => println!("s is empty!"),
    }
}

// *--- 1. 标签联合体 --- //
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String), // Tuple struct variant
    #[allow(dead_code)]
    ChangeColor (i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self { // * match将枚举拆开，self指Message
            Message::Quit => println!("Quit!"), // * match用=>来匹配
            Message::Move {x, y} => println!("Move to ({}, {}).", x, y),
            Message::Write(text) => println!("Text: {}.", text),
            Message::ChangeColor(_, _, _) => println!("Color Changed!"),
        }
    }
}

// *-- 2. Option联合体 --- //
// 泛型 T: 意味着它可以包任何东西
enum MyOption<T> {
    None,
    Some(T),
}