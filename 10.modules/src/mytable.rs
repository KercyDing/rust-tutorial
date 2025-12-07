#[derive(Debug)]
#[allow(dead_code)]
pub struct MyTable {
    pub name: String, // * 加上pub才可以被外界直接访问
    pub score: i32,
    id: u32, // 不可被直接访问
}

impl MyTable {
    pub fn new(name: String, score: i32, base_id: u32) -> MyTable {
        MyTable {
            name,
            score,
            id: MyTable::calc(base_id), // * 使用完全限定语法来访问calc函数
        }
    }

    fn calc(id: u32) -> u32 {
        id + 1
    }
}

pub fn write_something () {
    println!("The most handsome man in the world is:");
    kercy();
}

fn kercy () {println!("Kercy!")}
