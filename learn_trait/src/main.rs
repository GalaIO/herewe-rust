use std::fmt::{Display, Formatter};

fn main() {

    // trait可以叫特征，被用来向编译器描述某些特定类型拥有且能够被其他类型共享的功能
    // 简单来说就是可以定义一系列的行为，然后约束具体类型的行为，来实现多态，同样也可以用到泛型来约束具体类型需要满足的行为
    // 我们可以把相同的行为 抽象到trait中，这样在实际使用时，可以屏蔽具体对象，达到多态目的，关注行为而不是实例，就是面向接口编程

    // 定义了摘要的行为
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 可以给trait提供默认实现
    pub trait Author {
        fn get_author(&self) -> String {
            String::from("none")
        }
    }

    struct Article {
        content: String,
        author: String,
    }

    // 使用impl for语句实现summary
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("content: {}, author: {}", self.content, self.author)
        }
    }

    // 当使用默认实现的时候 可以提供空实现，也可以重载实现
    impl Author for Article {
    }

    let article = Article {
        content: String::from("test"),
        author: String::from("test"),
    };

    println!("article: {}", article.summarize());
    println!("author: {}", article.get_author());


    // 接下来我们看看trait怎么使用？具体就是入参、出参
    // 这个是简写
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // 这个是完整体，底层逻辑还是使用泛型，trait作为泛型约束条件 即trait bound，保证存在符合预期的行为实现
    pub fn notify2<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
    // 当存在多个约束时，使用+连接多个trait
    pub fn notify3<T: Summary+Display>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
    // 通过where重构泛型定义，把trait约束和泛型函数定义分开，比较清晰
    pub fn notify4<T>(item: T)
        where T: Summary + Display {
        println!("Breaking news! {}", item.summarize());
    }

    // 同样可以在返回值，定义也是类似的
    pub fn get_summary() -> impl Summary {
        Article {
            content: String::from("test"),
            author: String::from("test"),
        }
    }

    // 编写泛型版本的寻找最大值
    let v = vec![1, 2, 12, 3];
    println!("find largest: {}", find_largest(&v));
    println!("find largest: {}", find_largest2(&v));

    // 如果类型本身就有泛型呢？
    // 在定义方法的时候依然可以限制特定类型的行为，这个叫做覆盖实现
    // 也就是说 我们有条件地实现了方法，并不满足所有的类型指定
    struct Point<T> {
        x: T,
        y: T,
    }

    // 只有满足Display的约束类型 才能调用实现
    impl<T: Display> Point<T> {
        fn display(&self) {
            println!("display: {}", self.x.to_string());
        }
    }
}

// 因为在返回值返回 数字值而不是引用，所以入参的切片必须实现Copy，可以保证不会产生move语义，而是栈复制即可
// PartialOrd约束时为了可以比较，幸运的是 i32类型都实现了这些trait
fn find_largest<T: PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &x in list {
        if largest < x {
            largest = x
        }
    }
    largest
}

fn find_largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for x in list {
        if *largest < *x {
            largest = x
        }
    }
    largest
}
