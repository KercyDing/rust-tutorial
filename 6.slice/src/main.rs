fn main () {
    // *--- 1. 字符串切片 --- //
    let str1 = String::from("Hello, world!");
    let s = &str1[0..5]; // * 左闭右开
    println!("s: {}", s);
    let s = &str1[4..10];
    println!("s: {}", s);
    let s = &str1[4..]; // * 到最右侧
    println!("s: {}", s);
    let s = &str1[..7]; // * 从最左侧
    println!("s: {}", s);
    let s = &str1[..]; // * 全部
    println!("s: {}", s);

    // *--- 2. 数组切片 --- //
    let arr1 = [1, 3, 5, 7, 9, 11, 13, 15, 17];
    let a1 = &arr1[3..5];
    println!("a1: {:?}", a1);

    let mut arr2 = [1, 3, 5, 7, 9, 11, 13, 15, 17]; // * 注意这里要加mut，否则a无法被修改
    let a2 = &mut arr2[3..5]; // a是[7, 9]

    // arr = [0; 9]; // ERR: 这里arr被借用，无法进行修改
    a2[0] = 8;
    println!("a2: {:?}", a2);
    assert_eq!(a2, &[8, 9]); // * Rust创建了一个包含8和9的数组，借用它并伪装成切片。数组引用&[T; n]可以自动转换为切片&[T]
    println!("Success!");
}
