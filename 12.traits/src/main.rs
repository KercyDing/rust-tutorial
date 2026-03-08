use std::fmt;

fn main() {
    let str = "Kercy".to_string();
    let logger1 = ConsoleLogger;
    let logger2 = FileLogger {
        file_path: "~/path/saved.log".to_string(),
    };

    // *--- 1. 基本调用 --- //
    logger1.log(&str);
    logger2.log(&str); // 同一个方法，不同的行为 -> 多态

    // *--- 2. 默认实现 --- //
    // * 不需要在impl里写info/warn，trait里的默认实现自动可用
    logger1.info(&str);
    logger1.warn(&str);
    logger2.info(&str);
    logger2.warn(&str);

    // *--- 3. trait作为参数 --- //
    // * &impl Logger即"接受任何实现了Logger的类型"
    user_login(&logger1, "2023210000");
    user_login(&logger2, "2023210001"); // 两种logger都能传

    // *--- 4. 多个trait约束 --- //
    // * impl Logger + Resetable = 要求同时满足两个trait
    maintain(&logger2);
    // maintain(&logger1); // ERR: ConsoleLogger没有实现Resetable

    // *--- 5. Display trait --- //
    // * println!("{}", x) 会调用 Display trait 的 fmt 方法
    println!("{}", logger2);
    // println!("{}", logger1); // ERR: ConsoleLogger没有实现Display

    // * {:?} 用 Debug trait（可derive），{} 用 Display trait（必须手写）
    logger2.reset();
}

// *--- 1. 定义trait：声明能力 --- //
// * trait只声明方法签名，不写实现
trait Logger {
    fn log(&self, content: &str); // 每个impl必须实现

    // * 默认实现：trait里直接写函数体，impl可以免费继承，也可以覆盖
    // * 默认实现可以调用同trait里的其他方法
    fn info(&self, content: &str) {
        self.log(&format!("[INFO] {}", content)); // format!拼字符串但不打印，&转为&str传给log
    }

    fn warn(&self, content: &str) {
        self.log(&format!("[WARN] {}", content));
    }
}

trait Resetable {
    fn reset(&self);
}

// *--- 2. 实现trait：为具体类型履约 --- //
// * 一个impl块只能对应一个trait，不能合并写
struct ConsoleLogger;

struct FileLogger {
    file_path: String,
}

impl Logger for ConsoleLogger {
    fn log(&self, content: &str) {
        println!("{}", content);
    }
}

impl Logger for FileLogger {
    fn log(&self, content: &str) {
        println!("Write '{}' into path: {}", content, self.file_path);
    }
}

// * 一个类型可以实现多个trait，能力叠加，互不冲突
impl Resetable for FileLogger {
    fn reset(&self) {
        println!("The file in {} has been deleted!", self.file_path);
    }
}

// *--- 3. 标准库trait：Display --- //
// * #[derive(Debug)] 本质就是编译器自动写 impl Debug for ...
// * Display不能derive，必须手写
impl fmt::Display for FileLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileLogger({})", self.file_path) // 固定模板，只改write!里的内容
    }
}

// *--- 4. trait作为参数 & 多trait约束 --- //
fn user_login(logger: &impl Logger, user_id: &str) {
    logger.log(user_id);
}

// * impl Logger + Resetable: 参数必须同时拥有两种能力
fn maintain(logger: &(impl Logger + Resetable)) {
    logger.log("Starting maintaining...");
    logger.reset();
}
