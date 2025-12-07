fn main() {
    println!("---{}---", String::from("1. Create strings"));
    // * 1. 用String::from()来创建
    let mut s1 = String::from("Hello, world!");

    s1.push_str(" I'm Kercy"); // * push_str()追加字符串
    s1.push('.'); // * push()追加字符
    println!("{}", s1);

    // * 2. 用String::new()来创建空字符串
    let mut s2 = String::new();
    s2.push_str("I'm your father!");
    println!("{}", s2);

    // * 3. 用to_string()将 &'static str 来转化为字符串
    let s3 = "Who are you?".to_string();
    println!("{}", s3);

    println!("---{}---", String::from("2. Concatenate strings"));
    // * 1. 拼接
    let s4 = s2 + &s3; // * 这里s2所有权被转移，s3则是借用(Rust的 + 特性: String + &String)
    println!("{}", s4);

    // * 2. 宏拼接(推荐)
    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");

    let s8 = format!("{}-{}-{}", s5, s6, s7); // * format!()宏来拼接
    println!("{}", s8);

    println!("---{}---", String::from(""));

    let len_en = String::from("Hello").len();
    let len_zh = String::from("你好").len();
    println!("len of Hello: {}", len_en);
    println!("len of 你好: {}", len_zh);

    let hello = "Здравствуйте";
    // let answer = hello[0]; // ERR: String类型禁止下标索引(因为utf-8每个字符占用的字节数不同)
    // * 1. 按字符遍历
    print!("Chars: ");
    for c in hello.chars() {
        print!("{}", c);
    }
    println!();

    // * 2. 按字节遍历
    print!("Bytes: ");
    for b in hello.bytes() {
        print!("{}", b);
    }
    println!();

    // * 危险示范: 切片
    // * 这里的 [0..4] 是指前 4 个字节
    // * 俄语字母通常占 2 个字节，所以这会打印前两个字母
    let s = &hello[0..4];
    println!("Slice: {}", s);
    // let s1 = &hello[0..3];
    // println!("s1: {}", s1); // ERR: 这里取前3个字节，会从第二个俄文单词中间切开！就会发生panic
}
