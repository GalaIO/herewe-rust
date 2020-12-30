use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    dynamic_array();
    string_test();
    hashmap_test();
}

// hash映射 用来存储键值对，提高查询效率 非常常用
fn hashmap_test() {
    // 创建hashmap
    let mut map = HashMap::new();
    let s1 = "hello".to_string();
    // hash映射插入的值 对于非Copy的类型 都会move到映射中，随后无法继续使用
    // 可以存入引用类型，但是需要保证在声明周期内 原值都是有效的才行，直接move反而方便也支持修改
    // insert在插入式 会新增或者覆盖掉原有的kv关系
    map.insert(1, s1);

    // 查询hashmap值 使用for遍历
    let opt = map.get(&1);
    match opt {
        Some(t) => println!("map get {}", t),
        None => println!("map get none"),
    }
    for (k, v) in &map {
        println!("map k: {}, v: {}", k, v);
    }

    // 现在都是返回不可变引用，如果想修改v怎么办呢？需要返回可变引用
    // 使用entry().or_insert()会在为空时插入默认值，否则无法修改嘛，这时候返回可变引用可以修改v
    let v = map.entry(1).or_insert("world".to_string());
    v.push_str(" world");

    for (k, v) in &map {
        println!("map k: {}, v: {}", k, v);
    }
}

// string在任何一个编程语言中 都是一等公民，是一个非常常见的功能和场景
// 会针对字符串做一些处理和支持，比如utf-8、索引、字符串操作等等，底层大都类似动态数组
// 在rust中存在两种字符串类型 一个是String 一个是str字符串切片，一般都使用&str，它的不可变引用
fn string_test() {
    // 创建字符串
    let mut s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "hello".to_string();

    // 字符串拼接
    // 拼接字符串切片 比如 字符串字面量 或者s2[1..]
    s1.push_str(" world");
    s1.push_str(&s2[1..]);
    // 字符串使用+来操作 依然是字符串切片不可变引用，注意必须s1= 否则s1因为move无法继续使用
    s1 = s1 + &s2;
    // format!宏简化拼装过程
    println!("{}", format!("s1: {}, s2: {}", s1, s2));

    // 索引元素
    // string的底层是Vec<u8>实现的 不过对于底层可能是一个字节 可能是两个字节，也可能是字型簇等存储，
    // 所以除非确定是ANSII码否则最好别轻易使用切片语法
    // s1[1..]
    // 正确的是使用chars函数或者bytes获取unicode字符或者底层字节
    for c in s1.chars() {
        println!("char: {}", c);
    }
    for c in s1.bytes() {
        println!("byte: {}", c);
    }
}

// 动态数组，和数组不一样无法在一开始确定数组长度，所以无法在栈创建只能在堆中
fn dynamic_array() {
    // 使用vec的关联函数 来执行初始化操作，同时vec支持泛型所以要手动指定类型，
    // 当然也可以在随后push元素让编译器推导
    let mut v = Vec::new();
    v.push(1);

    // rust提供宏来简化创建过程
    let v2 = vec![1, 2, 3];

    // 读取vec中的值，直接使用索引返回的是值，这时候产生拷贝
    let num = v[0];
    // 这里是可以的 因为我们只是把值拷贝出来没有引用出现
    v.push(2);
    println!("v index 0: {}", num);

    // 读取vec中的值，使用&[] 返回的是该索引值引用，这时候指向和索引一样内存，属于借用语义
    let num = &v[0];
    // 因为使用索引获得了不可变引用，这时候没法传入push函数可变引用去修改数组
    // 你可能疑问 不可变引用是某个元素的 而可变引用是数组，这因为你是用的堆内存会因为扩容而修改，这时候可能引用错误内存地址，所以禁止
    // 报错 cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(2);
    println!("v index 0: {}", num);
    // 如果访问越界 会触发运行时panic需要注意
    // thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 100', learn_collections/src/main.rs:24:16
    // let num = &v[100];

    // 更好的方式是使用带错误码的结果，比如Option来判断是否存在或成功，rust和go一样也是错误码优先异常
    // 这时候返回的依然是引用
    let opt = v.get(100);
    match opt {
        Some(t) => {
            println!("get {} index num.", 100)
        },
        None => {
            println!("not found index {}.", 100)
        }
    };

    // 遍历数字中的值，非常方便的是可以使用in关键字自动遍历
    // 除了直接获取值以外，还可以获取引用和可变引用 &v 或 &mut v
    for i in &mut v {
        // 使用解引用来修改内存指向值
        *i += 10
    }
    for i in &v {
        println!("get num form v, {}", i);
    }

    // 遍历的时候最好不要 使用v会导致 move语义，vec在另外作用域最终释放无法在当前作用域使用了
    // for i in v {
    //     println!("get num form v, {}", i);
    // }

    // 同样动态数组在离开作用域后也会自动回收，调用的Drop函数对于对上内存，对于栈自动回收
}
