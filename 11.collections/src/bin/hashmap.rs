use std::collections::HashMap;

fn main(){
    let sub1 = String::from("Chinese");
    let num1 = 130;
    let sub2 = String::from("Math");
    let num2 = 135;

    let mut scores = HashMap::new();
    scores.insert(sub1, num1);
    scores.insert(sub2.clone(), num2.clone());

    println!("{:#?}", scores);

    // println!("sub1: {}, num1: {}", sub1, num1); // ERR: sub1 num1被插入HashMap，所有权被转移
    println!("sub2: {}, num2: {}", sub2, num2); // * sub2 num2采用clone所以所有权还在
    
    // *--- 1. 直接读取+match匹配
    let sub_name = String::from("Math");
    match scores.get(&sub_name) {
        Some(score) => println!("The score is: {}", score),
        None => println!("Can not find the subject!"),
    }
    
    // *--- 2. 使用copied()和unwrap_or()读取
    // * copied()将Option(&i32)转化为Option(i32)
    // * unwrap_or()如果是None则返回0
    let score = scores.get("Chinese").copied().unwrap_or(0);
    println!("Chinese score: {}", score);
    println!();

    // * 遍历: 由于HashMap是无需的，所以结果也是随机的
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //* 更新
    let mut stats = HashMap::new();
    stats.insert("a", 1);

    // *--- 场景A. 只有当key不存在时才插入
    stats.entry("a").or_insert(5); // a存在，不插入，返回1
    stats.entry("b").or_insert(5); // b不存在，插入，b=5

    println!("{:#?}", stats);

    // *--- 场景B. 计数器
    let text = "guess how many words do I have and I guess you can't";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Words count: {:#?}", words);
}