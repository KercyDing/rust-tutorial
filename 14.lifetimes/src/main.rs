#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // *--- 1. 基本生命周期 --- //
    // * 多个引用参数 + 返回引用时，编译器不知道返回值跟谁绑定，需要手动标注
    println!("The longer: {}", longer("Kercy", "Ding"));

    // * 'a 是约束，编译器会验证每个调用处是否满足
    // let result;
    // {
    //     let s2 = String::from("short");
    //     result = longer("long!", &s2);
    // } // ERR: s2在这里被释放，但result可能指向s2
    // println!("{}", result);

    // *--- 2. struct里的生命周期 --- //
    let novel = String::from("Call me Kercy. Some years ago...");
    let excerpt = Excerpt {
        content: &novel[..15], // 借用novel的一部分
    };
    println!("Excerpt: {}", excerpt.content);
    // * excerpt 不能活得比 novel 更久，否则引用悬空
}

// *--- 1. 函数生命周期 --- //
// * 'a 是生命周期参数，跟泛型<T>类似，只是标注引用之间的关系
// * 含义：s1、s2、返回值三者的引用生命周期一样长
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// *--- 2. struct生命周期 --- //
// * struct里存引用时，必须标注生命周期
// * 含义：Excerpt不能活得比它借用的content更久
struct Excerpt<'a> {
    content: &'a str,
}

// *--- 3. 省略规则 --- //
// * 只有一个引用参数时，编译器自动推断，不用手写'a
// * 例如下面这个函数，等价于 fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    &s[..1]
}
