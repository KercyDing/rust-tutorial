fn main() {
    // *--- 1. 栈数据的 Copy ---
    let x = 5;
    let y = x; // y只是拷贝了x的值
    println!("x: {}, y: {}", x, y);

    // *--- 2. 堆数据的 Move ---
    let s1 = String::from("Kercy");
    /*
        指针拷贝，所有权转移
        s1不再拥有string的所有权
     */
    let s2 = s1; 
    
    // println!("s1: {}", s1); // ERR: 移动之后所有权被转移

    println!("s2: {}", s2);

    // *--- 3. 显式 Clone ---
    let s3 = s2.clone(); // s3在堆上新开辟了内存
    println!("s2: {}, s3: {}", s2, s3)
}