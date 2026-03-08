#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

fn main() {
    // *--- 1. 函数泛型 --- //
    let list1 = [15, 9, 7, 1, 3, 5, 13, 11, 17];
    let list2 = ['b', 'v', 'e', 'u', 'z', 'r', 'n', 'q', 'm', 'q'];

    // * 同一个函数，处理不同类型
    println!("The largest of list1: {}", largest(&list1[..]));
    println!("The largest of list2: {}", largest(&list2[..]));

    // *--- 2. struct泛型 --- //
    let a = Point { x: 5, y: 10 }; // T 自动推断为 i32
    let b = Point { x: 1.0, y: 4.0 }; // T 自动推断为 f64
    // let c = Point { x: 5, y: 4.0 }; // ERR: 一个T只能代表一种类型，x和y必须同类型

    // * 多类型参数：T和U可以不同
    let d = MixedPoint { x: 5, y: 4.0 }; // T=i32, U=f64

    // *--- 3. impl泛型 --- //
    let e = Point::new(3, 7);
    let f = Point::new(1.5, 2.5);
    // * 特化方法：只有 Point<f64> 才能调用 distance
    // e.distance(); // ERR: Point<i32>没有distance方法
    println!("f distance: {}", f.distance());

    // *--- 4. where语法 --- //
    print_largest(&list1[..]);
    print_largest(&list2[..]);
}

// *--- 1. 函数泛型 --- //
// * <T: PartialOrd> = trait bound，约束T必须能比大小
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// *--- 2. struct泛型 --- //
struct Point<T> {
    x: T,
    y: T, // x和y必须同类型，因为都是T
}

// * 多类型参数：x和y可以是不同类型
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// *--- 3. impl泛型 --- //
// * impl<T> Point<T> = 对所有类型的Point实现方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

// * impl Point<f64> = 只给Point<f64>实现方法，其他类型没有
impl Point<f64> {
    fn distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// *--- 4. where语法 --- //
// * 当trait bound太长时，用where换行写更清晰，功能完全一样
// * 等价于 fn print_largest<T: PartialOrd + Display>(list: &[T])
fn print_largest<T>(list: &[T])
where
    T: PartialOrd + Display,
{
    println!("The largest: {}", largest(list));
}
