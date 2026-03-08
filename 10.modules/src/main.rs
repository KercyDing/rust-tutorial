mod mytable; // * 在当前目录下找到mytable，把它挂载成子模块
// use crate::mytable::{MyTable, write_something}; // * crate代表根目录(main.rs)
use mytable::{MyTable, write_something}; // * 默认不写就是当前路径(或者前面加上self::)
// use super::mytable::{MyTable, write_something}; // ERR: super则代表上级目录开始找(没有了)

// * 定义两个内联模块，防止函数名冲突
mod student {
    pub fn speak() {
        println!("I love study!")
    }
}

mod teacher {
    pub fn speak() {
        println!("You're so stupid!")
    }
}

fn main() {
    let t = MyTable::new(String::from("Kercy"), 100, 2023215417);
    println!("Table: {:#?}", t);
    // println!("ID is: {}", t.id); // ERR: id是私有变量无法访问

    write_something();

    // * 这样就可以区分
    student::speak();
    teacher::speak();
}
