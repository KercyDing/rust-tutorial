fn main() {
    // let v1: Vec<i32> = Vec::new(); // ERR: 不加mut的vector不能添加元素
    #[allow(clippy::vec_init_then_push)]
    {
        let mut v1: Vec<i32> = Vec::new(); // * 使用new函数创建
        v1.push(3);
    }

    let mut v2 = vec![1, 3, 5]; // * 使用vec!创建

    let mut v3: Vec<i32> = Vec::with_capacity(10); // * 使用with_capacity()来预分配容量
    v3.push(0);

    let first = &v2[0];
    // v2.push(2); // ERR: 这里由于first借走了v2[0]，所以v2不能再扩容
    println!("first of v2: {}", first);

    match v2.get(100) {
        Some(third) => println!("The 100th element is: {}", third),
        None => println!("There is no 100th element."),
    }

    println!("--- 1.Iteration ---");
    for i in &v2 {
        // * i 的类型是 &i32
        // * 解引用 *i 才能拿到值进行计算
        println!("{}", *i + 50);
    }

    println!("--- 2.Mut Iteration ---");
    for i in &mut v2 {
        *i += 50;
    }
    println!("vector updated: {:#?}", v2);

    let row = vec![
        Sheet::Int(3),
        Sheet::Float(3.5),
        Sheet::Text(String::from("Hello!")),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
#[allow(dead_code)]
enum Sheet {
    Int(i32),
    Float(f64),
    Text(String),
}
