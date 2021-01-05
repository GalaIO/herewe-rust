fn main() {
    // 所有编程语言都会致力于高效地处理重复概念，在rust可以使用泛型处理重复代码
    // 泛型是具体类型或者其他属性的抽象替代。在编写代码时，可以直接描述泛型的行为，
    // 或者与其他泛型产生的联系，而不需要知道编译和运行时的具体类型
    // 当然当你使用泛型的时候，对泛型行为最好有限制，比如+操作，本身必须满足一定限制，所以泛型一般和trait结合使用

    // 泛型的语法很简单，类似java或者c++，支持对方法、类型进行泛型定义
    // 在方法名或者类型名后跟<T>表示泛型类型，也可以定义多种比如<T, E, U>
    let v1 = vec![1, 2, 3, 4];
    // println!("largest: {}", find_largest(&v1));

    // 定义结构体
    struct Point<T, U> {
        x: T,
        y: U,
    }

    // 为什么impl后要跟泛型定义呢，因为rust还支持另外一种定义，可以直接定义特定类型的行为，所以如果定义泛型行为需要跟泛型定义
    impl<T, U> Point<T, U> {
        fn new(x: T, y: U) -> Point<T, U> {
            Point{
                x,
                y,
            }
        }

        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<i32, i32> {
        fn x2(&self) -> i32 {
            self.x * self.x
        }
    }

    // 最后是泛型的优化，对于rust的泛型不是在运行时处理，而是在编译器决定的，rust使用的是单态化monomorphization
    // 会在编译器编译成确定的类型代码，所以性能与非泛型一致

}

// fn find_largest<T>(v: &[T]) -> T {
//     let mut tmp = v[0];
//
//     for &x in v {
//         if x > tmp {
//             tmp = x
//         }
//     }
//
//     return tmp
// }
