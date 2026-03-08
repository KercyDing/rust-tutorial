fn main() {
    let mut rec1 = Rectangle {
        width: 2,
        height: 5,
    };

    println!("rec1 info: {:#?}", rec1);
    // println!("rec1 area: {}", Rectangle::area(&rec1)); // 完全限定语法(::)，不推荐
    println!("rec1 area: {}", rec1.area()); // * 方法语法(.)，Rust自动解引用，等价于(&rec1).area()

    rec1.scale(2); // * 等价于(&mut rec1).scale(2)
    // Rectangle::scale(&mut rec1, 2);
    println!("rec1 info: {:#?}", rec1);

    let rec2 = Rectangle::new(4, 7); // * 必须用完全限定语法，因为new函数没有self参数
    println!("rec2 info: {:#?}", rec2);

    rec2.delete();
    // println!("rec2 info: {:#?}", rec2); // ERR: rec2已经被销毁
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

// * impl块中的代码服务于Rectangle
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        // * &self只读借用
        self.width * self.height
        // &self.width * &self.height 也行，Rust自动将指向整数的引用相乘带上了解引用，相当于(*self.w) * (*self.h)
    }

    fn scale(&mut self, factor: u32) {
        // * &mut self可变借用
        self.width *= factor;
        self.height *= factor;
    }

    fn delete(self) {
        // * self是夺取所有权，函数调用之后销毁 -> consume模式
        println!("It has been deleted!");
    }
}
