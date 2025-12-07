fn main() {
    // * if表达式
    let n = 5;
    if n > 3 {
        println!("n > 3!");
    } else {
        println!("n <= 3!");
    }

    let condition = true;
    let number = if condition {5} else {6}; // * 类似三元运算符
    println!("number: {}", number);

    // * loop循环
    // * loop通过break来返回值
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // 类似于C++ return
        }
    };
    println!("Result: {}", result);

    // * while循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // * for循环
    let a = [10, 20, 30, 40, 50, 60];
    for n in a.iter() { // * iter()返回引用，避免争夺所有权
        println!("Value: {}", n);
    }
    println!();

    for n in (1..6).rev() { // * (1..6)，左闭右开，从1-5。rev()反转，即5-1
        println!("n: {}", n);
    }
    println!();

    for n in 1..=5 { // * 1..=5，左闭右闭，从1-6
        println!("n: {}", n);
    }
}
