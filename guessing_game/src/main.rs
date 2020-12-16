use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 使用println!宏来输出字符串到控制端
    println!("Guess the number!");

    // 调用rand方法生产范围内的随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("the secret number is {}", secret_number);

    loop {
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

        // rust允许使用同一个变量名 再次定义为其他类型，是rust的隐藏机制
        // trim会去掉首尾的空白字符，比如空格、换行等等
        // 使用match匹配parse返回的结果，如果报错会直接跳过忽略
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match 表达式由多个分支组成，每个分支包含一个用于匹配的模式，以及匹配到的执行代码
        // 匹配代码执行完后就会退出，不再继续执行
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        };
    }
}
