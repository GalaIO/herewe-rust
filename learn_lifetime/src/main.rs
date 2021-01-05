use std::fmt::Display;

fn main() {

    // 总所周知，rust没有垃圾回收，不需要主动释放内存，通过编译器在实例不再使用时(作用域结束)执行Drop
    // 但是有一个问题，如果持有一个实例的引用，那么有可能在实例被释放还有可能使用么？也就是出现悬垂引用，值得注意的是rust中不存在null值，如果出现可能null的情况编译不会通过，保证引用有效
    // 这时候要提到 rust的生命周期的概念，生命周期是针对引用而言，保证引用在实例有效的作用域内有效，
    // 否则会报错 xx does not live enough，实例有效范围不足
    // 到目前为止 还没有接触到生命周期，因为在一些场景可以通过编译器自动推导
    // 但是当多个引用的生命周期以不同方式相互关联时，必须手动标注声明周期，比如多个入参、结构体中的引用成员等等
    // 这时候必须标注来多个引用之间的关系，比如是相同的生命周期还是不同的，来确保运行时实际使用的引用一定有效

    // 如下程序无法通过编译，报错`x` does not live long enough
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //
    //     println!("r: {}", r);
    // }

    // rust是如何发现可能存在悬垂引用呢？是通过借用检查器 borrow checker，保证被引用的对象存在范围大于引用者
    // 如何标注生命周期呢？分别说明 函数定义、结构体定义、方法实现的生命周期标注

    // 函数定义中使用生命周期，其实就是泛型定义，表示生命周期的约束，拥有多长的生命标记，同时逗号可以跟T，比如<'a, 'b, 'c, T>，这时候增加泛型约束最好使用where语法
    // 生命周期标记通过'开头，随后跟若干的小写字母，在随后的入参生命周期和出参生命周期 使用类似 &'a i32 或者 &'a mut i32表示
    // fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
    //     if x.len() > y.len() {
    //         return x;
    //     }
    //     y
    // }

    // 上面的例子我们给两个入参和一个出参定义了不同的生命周期，但是不能编译通过，报错
    // note: ...the reference is valid for the lifetime `'c` as defined on the function body at 26:24...
    //   --> learn_lifetime/src/main.rs:26:24
    //    |
    // 26 |     fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
    //    |                        ^^
    // note: ...but the borrowed content is only valid for the lifetime `'a` as defined on the function body at 26:16
    // 意思是 我需要c的生命周期，你给我个a的，，什么意思。。。
    // 还记得之前提过，生命周期其实是表示各个引用的关系，也就是说需要保证谁和谁一样的存在周期，谁和谁不一样，谁和谁要保证有效期都存在，上面没有这样的信息
    // 如果把返回值修改为 &'a str，这时候编译失败 y: &'b str，this parameter and the return type are declared with different lifetimes...
    // 因为编译器不知道是x还是y返回，不能说保证返回和x保证一样的声明周期就好了，可能返回y

    // 正确的标记如下，大家生命周期一样，也就是说，大家都不能先失效，比如原实例回收，或者引用回收
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            return x;
        }
        y
    }
    println!("longest: {}", longest("123", "1234"));

    // 下面使用也报错，`y` does not live long enough
    // y的实例被提前回收了
    // {
    //     let x = String::from("1234");
    //     let res;
    //     {
    //         let y = String::from("123");
    //         res = longest(&x, &y);
    //     }
    //     println!("longest: {}", res);
    // }

    // 下面没问题，虽然引用被回收，但是实例还在有效期
    {
        let x = String::from("1234");
        let y = String::from("12345");
        let res;
        {
            let ref_y = &y;
            res = longest(&x, ref_y);
        }
        println!("longest: {}", res);
    }

    // 结构体中定义生命周期标注
    struct Article<'a> {
        content: &'a str,
        author: &'a str,
    }

    // 值得庆贺的是，所有结构体的方法都不需要声明生命周期，可以被编译器自动推断，但是impl语法上还是要显示声明，和泛型语法类似
    // 这里额外说一下为什么可以自动推断，因为方法都持有自己的引用&self 或者 &mut self，这时候有一个声明周期是知道的，那么其他返回声明周期和该实例保持一致即可
    // 就像 fn print(&'a self, x: &'b str) -> &'a str，但是如果返回值使用了x，那么推断后依然报错，x: &'b str this parameter and the return type are declared with different lifetimes...
    // 让你标明返回值和x也是有效的
    impl<'a> Article<'a> {
        fn print(&self, c: &str) -> &str {
            println!("input: {}", c);
            self.content
        }
    }

    // 下面编译失败，报错`test` does not live long enough
    // 因为指示编译器 实例的生命周期与成员引用指向的实例的生命周期要一致，但是成员引用的实例已经释放，所以报错，成员变成悬挂引用
    // let article;
    // {
    //     let test = String::from("test");
    //     article = Article {
    //         content: &test,
    //         author: &test,
    //     };
    // }
    // println!("article: {}", article.content);

    // 阐明下编译器自动推断 生命周期的规则：
    // 1.每一个引用入参参数都会有独立的生命周期参数，比如fn foo<'a, 'b>(&'a i32 x, &'b i32 y)
    // 2.当只存在一个入参时，这个生命周期会赋予所有的输出生命周期参数
    // 3.当有多个输入生命周期参数时，其中一个是&self 或 &mut self时，self的生命周期会赋予所有的输出生命周期参数
    // 当匹配玩3个规则，所有的入参出参没有合适的生命周期时，会报错让用户自行设置，当然不管编译器推断还是自行设置生命周期也有可能出错，具体问题具体判断即可

    // 静态声明周期'static，静态生命周期在整个程序的执行期都有效，比如所有的字符串字面量都是'static
    // 注意静态生命周期也别滥用，在必须是用，尤其是可变引用要更小心，存在数据竞态；

    // 最后给一个同时使用泛型、trait约束、生命周期的例子
    fn longest_with_ann<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display {
        println!("ann: {}", ann);
        if x.len() > y.len() {
            return x;
        }
        y
    }
}
