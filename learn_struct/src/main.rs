
// 定义结构体，注意成员定义和类型
// 注意这里面的成员都是变量，而不是引用
// 所以该结构体拥有这个成员的所有权，当然也可以声明为引用
// 但是由于引用没有所有权 所以必须申请声明周期，保证该结构体实例中引用数据的有效期不短于实例本身，否则就会导致悬挂引用
// 申明注解，增加Debug注解可以使用{?:}打印结果
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 初始化实例，把两个String的所有权交给结构体
fn build_user(name: String, email: String) -> User {
    User{
        // 同名变量可以简化赋值
        name,
        email,
        active: true,
        sign_in_count: 0,
    }
}

impl User {

    // 关联函数 有点像类的静态函数
    // 可以直接调用，但是调用必须使用:: 有点像c++的命名空间调用
    // 比如 User::buildDefault()
    fn buildDefault() -> User {
        build_user(String::from("xiaoguo"), String::from("1@123.com"))
    }

    // 在impl块中增加成员函数，这时候和python类似必须加self变量，
    // 当然可以是self获取所有权move后原变量无法再使用
    // 也可以是不可变的借用&self 或者是可变借用&mut self
    fn incrSignInCount(&mut self) {
        self.sign_in_count += 1;
    }
}

// 元组结构体，非常方便定义一些简单结构
struct Color(i32, i32, i32);


fn main() {
    println!("Hello, world!");
    let black: Color = Color(0, 0, 0);
    println!("color: {}", black.0);

    let mut  user = build_user(String::from("xiaoguo"), String::from("dot@dot.com"));
    user.incrSignInCount();
    println!("user name: {}", user.name);
}
