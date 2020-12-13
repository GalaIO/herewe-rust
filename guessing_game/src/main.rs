use std::io;

fn main() {
    // 使用println!宏来输出字符串到控制端
    println!("Guess the number!");
    println!("Please input your guess...");

    // 使用内置的String调用关联函数(不属于某实例，而是直接通过域调用)产生空白的字符串
    // 返回值绑定到可变的变量，mut guess可变表示可以修改
    // 对于rust默认时不可变，rust会检查是否会修改内存
    let mut guess = String::new();

    // 使用io::stdin的关联函数来读取控制行的输入，必须传入可变的引用
    // 引用会降低在函数间传递的拷贝开销，但是rust不同的事，引用也是默认不可变，无法修改内存
    // 这时候必须指定mut &方式
    io::stdin().read_line(&mut guess)
        // 在rust错误处理也不是以异常为主，而是主动判断和处理错误
        // 不过一个好处是，rust可以通过简单的链式调用来处理，返回io::Result
        // Result是一个枚举类型，枚举由一系列固定值组合而成，枚举的值也被称为变体，比如Ok和Err
        // 如下的expect函数当枚举是Err的时候回panic中断程序打印传入msg和错误信息，Ok的时候会返回结果
        .expect("Failed");

    // 使用println!宏来输出字符串到控制端，可以通过{}来输出参数
    println!("You guessed: {}", guess);
}
