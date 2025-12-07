fn main () {
    println!("Hello, world!");

    let a = 5;
    println!("The value of a: {}, and address: {:p}", a, &a);
    // * 这里采用“变量遮蔽”，a指向新的内存空间
    let a = a + 1;
    println!("The value of a: {}, and address: {:p}", a, &a);
    let a = "Hello";
    println!("The value of a: {}, and address: {:p}", a, &a);
    println!();

    {
        let mut b = 10;     // * b加上了mut
        println!("The value of b: {}", b);

        // a = 10;      // ERR a的值不能改变
        b = 20;         // * b的值可以修改
        println!("The new value of b: {}", b);
        println!("");
    }
    // println!("The value of b is: {}", b);        // ERR b不在作用域中

    let c = a;     // * c是i32类型，可以copy
    println!("The value of a: {}", a);
    println!("The value of c: {}", c);

    let str1 = String::from("Hello World");
    println!("The str1 is: {}", str1);
    let str2 = str1;
    // println!("The str1 is: {}", str1);      // ERR str1所有权已被借用
    println!("The str2 is: {}", str2);
}
