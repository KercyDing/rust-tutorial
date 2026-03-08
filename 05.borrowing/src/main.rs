fn main() {
    // *--- 1. 不可变引用 ---
    let s1 = String::from("Hello World");
    let len = calc_length(&s1); // 借用s1，不转移所有权
    println!("The length of {}: {}", s1, len); // s1仍然可用

    // *--- 2. 可变引用 ---
    let mut s2 = String::from("Kercy");
    change(&mut s2);
    println!("s2: {}", s2);

    // *--- 3. 可变引用的限制：同一作用域只能有一个 ---
    let mut s3 = String::from("Kercy");
    let r1 = &mut s3;
    // let r2 = &mut s3; // ERR: 不能同时有两个可变引用
    println!("r1: {}", r1);

    // *--- 4. 不可变引用和可变引用不能共存 ---
    let mut s4 = String::from("Kercy");
    let r1 = &s4;
    let r2 = &s4; // 多个不可变引用可以共存
    println!("r1: {}, r2: {}", r1, r2);
    // r1和r2作用域结束

    let r3 = &mut s4; // 不可变引用已经用完，所以可以创建可变引用
    // println!("r1: {}", r1); // ERR: 不可变引用和可变引用不能共存
    println!("r3: {}", r3);

    // *--- 5. 悬垂引用 ---
    // let ref_to_nothing = dangle(); // ERR: 函数返回局部变量的引用会造成悬垂引用
    let s5 = no_dangle();
    println!("s5: {}", s5);
}

/// 计算字符串长度，通过借用避免所有权转移
#[allow(clippy::ptr_arg)] // 教学用，演示 &String 借用
fn calc_length(s: &String) -> usize {
    s.len()
    // s离开作用域，但因为它不拥有所有权，所以什么也不会发生
}

/// 通过可变引用修改字符串
fn change(s: &mut String) {
    s.push_str(", Hello!");
}

// fn dangle() -> &String { // ERR: 返回局部变量的引用
//     let s = String::from("hello");
//     &s // s在函数结束后被释放，引用会指向无效内存
// }

// 直接返回String，转移所有权
#[allow(clippy::let_and_return)]
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 所有权被移出，不会被释放
}
