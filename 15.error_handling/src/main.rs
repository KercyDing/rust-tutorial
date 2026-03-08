#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // *--- 1. unwrap系列 --- //
    let n1: i32 = "42".parse().unwrap(); // * 成功取值，失败直接panic（别在生产代码用）
    let n2: i32 = "abc".parse().unwrap_or(0); // * 失败就用默认值
    let n3: i32 = "abc".parse().unwrap_or_else(|e| { // * 失败时可以拿到错误做点事
        println!("Error: {}", e);
        0
    });

    // *--- 2. match处理 --- //
    // * 必须处理所有分支，最严谨
    let result1: Result<i32, _> = "abc".parse();
    match result1 {
        Ok(n) => println!("Succeed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // *--- 3. if let简写 --- //
    // * 只关心一个分支时的语法糖，比match简洁
    let result2 = "24".parse::<i32>(); // parse::<i32>() 是turbofish语法，指定解析类型
    if let Ok(n) = result2 {
        println!("Succeed: {}", n);
    } else {
        println!("Error!");
    }

    // *--- 4. is_ok / is_err --- //
    // * 只判断成功失败，不关心具体值
    let result3: Result<i32, _> = "24".parse();
    if result3.is_ok() {
        println!("Succeed!");
    }

    // *--- 5. map / and_then / or_else 链式处理 --- //
    // * map: Ok时变换值（闭包返回普通值）
    let result4 = "24".parse::<i32>().map(|n| n * 2); // Ok(48)

    // * and_then: Ok时继续做可能失败的操作（闭包返回Result）
    let result5 = "24".parse::<i32>().and_then(|n| {
        if n > 100 { Err(n.to_string().parse::<i32>().unwrap_err()) } else { Ok(n * 2) }
    });

    // * or_else: Err时尝试补救（闭包返回Result）
    let result6 = "abc".parse::<i32>().or_else(|_| "42".parse::<i32>()); // Ok(42)

    // *--- 6. ? 操作符 --- //
    // * 在main里调用返回Result的函数，仍然需要处理错误
    let n4 = parse_number("21").unwrap_or(0);
    match parse_number("abc") {
        Ok(n) => println!("Succeed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // *--- 7. Option --- //
    // * Option<T> = 有值(Some) 或 没值(None)，和Result共用同一套方法
    // * 区别：Option表示"有没有"，Result表示"成不成功"（带错误信息）
    let names = vec!["Kercy", "Alice", "Bob"];
    let found: Option<&&str> = names.iter().find(|&&n| n == "Alice");

    if let Some(name) = found {
        println!("Found: {}", name);
    }

    // * Option和Result互转
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("值不存在"); // Option -> Result

    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok(); // Result -> Option（丢掉错误信息）
}

// *--- ? 操作符 --- //
// * ? 遇到Err就直接return Err给调用方，遇到Ok就取出值继续
// * 等价于 match s.parse() { Ok(v) => v, Err(e) => return Err(e) }
// * ? 只能在返回Result的函数里用
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}
