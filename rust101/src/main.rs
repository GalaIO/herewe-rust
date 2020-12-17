mod learn_type;

// 常量必须使用const声明，同时必须指定类型，而且常量无法修改，无法声明为mut
const MAX_RANGE: u64 = 100_000;

fn main() {
    let x = 6;
    // Cannot assign twice to immutable variable [E0384]
    // x = 7;

    // 声明可变x 可以在初始化后继续修改该值
    let mut x = 7;
    x = 9;

    // 通过let声明同名变量 对之前变量进行隐藏
    let x = 'a';
    println!("Hello, world!");
    println!("fbi: {}", fib(2));
}

fn fib(n: i32) ->i32 {
    if n == 0 {
        return 0
    }

    let mut x = 0;
    let mut y = 1;
    for i in (2..n+1).rev() {
        let z = x + y;
        x = y;
        y = z;
    }
    y
}
