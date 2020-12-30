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

    // 记住所有权规则：
    // 1.每一个值都有一个对应的变量作为所有者；
    // 2.在同一时间内或者同一个作用域内，值有且仅有一个所有者；
    // 3.当所有者离开自己的作用域，它持有的值就会自动被释放掉；

    let mut s = String::from("hello");
    s.push_str(", world");
    // 会创建s的浅拷贝(默认)，s包含只想内存的字符串指针、字符串长度、字符串容量
    // s1也类似不过s有指向堆指针，这时候不会随着栈结束而释放，
    // 需要进行主动堆内存释放，所以必须在作用域结束调用drop函数进行清理，
    // 同时指针拷贝导致可能重复清理，所以必须move到s1，这时候s无法使用
    // s将对象move到s1，随后不能再使用s，会导致保存
    // 在rust中，如果一个类型或者类型包含的成员 实现了Drop的trait，那么这个类型在赋值时就会move
    // 如果一个类型实现Copy的trait，那么就可以保留在栈上，赋值会导致拷贝，不会move依然可以使用
    // 当然实现Copy和Drop时冲突的不可能同时存在
    // 常见的整数、字符、浮点、只有copy的元组都是栈存储实现Copy，对于包含指针引用的都是堆存场实现Drop
    let s1 = s;
    // when use s, get error value borrowed here after move
    // println!("{}", s);
    println!("{}", s1);

    let x = 5;
    // 这时候发生拷贝，该类型都是在栈上随着调用结束自动释放，不存在释放堆内存
    let y = x;
    println!("{}", x);

    // 这里使用了clone申请新的堆内存对象，然后通过返回值move到s2，s1不会move到s2
    // 所以不会导致一个变量引用多个内存地址的问题
    let s2 = s1.clone();
    println!("s1: {}, s2:{}", s1, s2);

    // 在函数传递中的move行为也是一样的，move后不能使用，同时管理问题也交给函数
    // 所有权交给函数的入参，也就是说所有权发生了转移
    take_ownership(s2);
    // 这时候继续使用s2报错 value borrowed here after move
    // println!("s2: {}", s2);
    take_copy(x);

    // 如果在本函数还需要继续操作s1，只能move到函数入参后，在通过返回值move回s1
    // 所有权在函数入参和返回值进行转移
    let mut s1 = take_ownership_v2(s1);

    // 对于频繁在函数转移所有权回 很麻烦，rust通过引用来解决这个问题
    // 引用时会生成一个指针指向原值，同时不会发生所有权转移，引用的传递也不会影响原值，所以引用又叫借用，只是暂时借用不影响所有权
    // 引用默认也是不可变的，同时可以创建多个不可变的引用
    let bs1 = &s1;
    borrow_ownership(&s1);
    // 下面代码不能编译成功 cannot borrow `s1` as mutable more than once at a time
    // rust不允许 申请多次可变引用，这样会导致不可预见数据竞争race，当前有且只有一个可变引用操作数据
    // {
    //     let r2 = &mut s1;
    //     let r3 = &mut s1;
    //     println!("r2:{}, r3:{}", r2, r3);
    //     // 对于入参也一样，也会有多次可变引用问题
    //     // borrow_mut_ownership(&mut s1);
    //     // println!("r2:{}", r2);
    // }
    // 下面代码不能编译成功 cannot borrow `s1` as mutable because it is also borrowed as immutable
    // rust不允许可变引用和不可变引用同时出现，这样会导致不可预见的数据修改，即只读时候禁止操作数据
    // {
    //     let r1 = &s1;
    //     let r2 = &mut s1;
    //     let r3 = &mut s1;
    //     println!("r1:{}, r2:{}, r3:{}", r1, r2, r3);
    // }

    // 切片，其实是一种不可变引用，字符串的切片类型为&str 其他数组的切片类型是 &[type]
    // 因为是引用所以必须带&前缀，随后跟索引指定子数组
    let mut hw = String::from("hello world!");
    // 创建字符串切片，必须使用&v[..]，这是一个完整字符的数组，类型是&str
    // 可以增加前后索引指定子串切片，比如 &v[1..10] &v[..10] &v[1..]
    let s1 = &hw[..];
    let w = findFirstWord(&hw[..]);
    // 因为w已经是不可变引用，所以不能调用clear传入可变引用了
    // hw.clear();
    println!("first word: {}", w);

    let arr = [1, 2, 3, 4];
    // 如下是数组类型 [i32; 4]的切片，类型为 &[i32]
    let a1 = &arr[..];
}

fn findFirstWord(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    return s
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

fn take_ownership(s: String) {
    println!("take_ownership, s:{}", s);
}

fn take_ownership_v2(s: String) -> String {
    println!("take_ownership_v2, s:{}", s);
    // 重新返回s的所有权
    return s
}

fn borrow_ownership(s: &String) {
    println!("borrow_ownership, s:{}", s);
}

fn borrow_mut_ownership(s: &mut String) {
    println!("borrow_ownership, s:{}", s);
}

fn take_copy(s: i32) {
    println!("take_copy, s:{}", s);
}
