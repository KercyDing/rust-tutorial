fn main() {
    // *--- 1. 元组Tuple ---
    {
        let tup = (6.4, 'g', "kercy");

        let x = tup.0; // 访问元素
        let y = tup.1;
        let z = tup.2;

        print!("x: {} ", x);
        print!("y: {} ", y);
        print!("z: {} ", z);
        println!();

        let (a, b, c) = tup; // * 解构
        println!("a: {}, b: {}, c: {}", a, b, c);
        println!();
    }

    // *--- 2. 数组Array ---
    {
        // 1. 显示列出元素
        let arr1 = [1, 2, 3, 4, 5];
        println!("The first of arr1: {}", arr1[0]);

        /*
            2. 初始化重复值
            语法: [初始值; 长度]
        */
        let arr2 = [3; 5];
        println!("The second of arr2: {}", arr2[1]);

        let arr3 = arr2; // 这里是clone(copy的超集)而不是move
        println!("The third of arr2: {}, and arr3: {}", arr2[2], arr3[2]);

        // println!("The 6th of arr1: {}", arr1[5]); // ERR 数组越界导致panic

        // * 数组解构
        let [_a, _b, _c, _d, _e] = arr1; // 全部解构
        let [_x, _, _y, _z, _] = arr1; // 单个不解构的用_代替
        let [_head, .., _tail] = arr1; // 连续不解构的用..代替
        let [_first, _head, ..] = arr1;
    }
}
