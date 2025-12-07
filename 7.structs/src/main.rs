fn main() {
    let user1 = User {
        username: String::from("kercy"), // * 必须要显示声明变量名
        email: String::from("dkx215417@gmail.com"),
        sign_in_account: 2,
        active: true,
    };

    // * 结构体只能在外部加上mut，内部变量不能加。加了之后内部所有变量都为mut
    // * -> 可变性属于变量绑定的属性，而不属于数据类型的属性
    let mut user2 = build_user(String::from("Alice"), String::from("iloveyou@gmail.com"));
    user2.active = false;

    let user3 = User {
        username: String::from("Marry"), // 这里只修改username
        ..user1 // * Rust更新语法(..)，将除了username以外的其他字段直接从user1里面拿
    };

    let black = Color(0, 0, 0);

    #[allow(unused_variables)]
    let sub = AlwaysEqual;

    // println!("User1 info: {:?}", user1); // ERR: 这里报错是因为user1.email被user3的更新语法(..)移动走
    println!("User1 username: {}", user1.username);
    println!("User2 info: {:?}", user2); // * 非美化输出，一行显示
    println!("User3 info: {:#?}", user3); // * {:#?}美化输出，自动缩进换行

    println!("Color info: {:#?}", black);
}

// *--- 1. 具名结构体 --- //
#[derive(Debug)] // * 支持打印
#[allow(dead_code)] // * 让编译器别管有没有用
struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool,
}

// *-- 2. 元组结构体 --- //
#[derive(Debug)]
#[allow(dead_code)]
struct Color(i32, i32, i32);

// *--- 3. 单元结构体 --- //
struct AlwaysEqual;

// * 实例化函数
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_account: 1,
        active: true,
    }
}
