// 使用 newtype 模式实现类型安全和抽象
//  - newtype 模式可以：
//      - 用来静态的保证各种值之间不会混淆并标明值的单位
//      - 为类型的某些细节提供抽象能力
//      - 通过轻量级的封装来隐藏内部实现细节
// 使用类型别名创建类型的同义词
//  - Rust 提供了类型别名的功能：
//      -  为现有类型生产另外的名称【同义词】
//      -  并不是一个独立的类型
//      -  使用 type 关键字
//  - 主要用途：减少代码字符重复
// Never 类型
//  - 有一个名为 ! 的特殊类型：
//      - 他没有任何值，行货称为空类型【empty type】
//      - 我们倾向于叫他 never 类型，因为他在不返回的函数中充当返回类型
//  - 不返回值的函数也叫做发散函数【diverging function】
// 动态大小和 Sized Trait
//  - Rust 需要在编译时确定为一个确定类型的值分配多少空间
//  - 动态大小的类型【Dynamically Sized Types，DST】的概念：
//      - 编写代码时使用只有在运行时才能确定大小的值
//  - str 是动态大小的类型，注意不是 &str，只有运行时才能确定字符串的长度
//      - 下列代码无法正常工作
//          let s1: str = "Hello";
//      - 使用 &str 来解决
//          - 存的是 str 的地址和 str 的长度
//  - Rust 使用动态大小类型的通用方式
//      - 附带一些额外的元数据来存储动态信息的大小
//          -使用动态大小类型是总会把它的值放在某种指针后边
//  - 另一种动态大小的类型 Trait
//      - 每个 trait 都是一个动态大小的类型，可以通过名称对其进行引用
//      - 为了将 trait 用作 trait 对象，必须将它放置在某种指针之后
//          - 例如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait>）之后
//  - Sized Trait
//      - 为了处理动态大小的类型，Rust 提供了一个 Sized trait 来确定一个类型的大小在编译时是否已知
//          - 编译时可计算出大小的类型会自动实现这一 trait
//          - Rust 还会为每一个泛型函数隐式的添加 Sized 约束
//      - 默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法接触这一限制
//          - ?Sized trait 约束
// fn generics<T>(t: T) {}
// fn generics<T: Sized>(t: T) {}
// fn generics<T: ?Sized>(t: &T) {}，？只能用于 Sized 上边


type Kilomoters = i32;

// 类型很复杂的时候，这么优化

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {

// }

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//     Box::new(|| println!("hi"))
// }

type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}


fn advanced_types_demo() {
    let x: i32 = 5;
    let y: Kilomoters = 5;
    println!("x + y = {}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("Hi"));
    let f: Thunk = Box::new(|| println!("Hi"));

    // let s1: str = "Hello";
    let s1: &str = "Hello";

    let guess = "";
    
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // continue 就是 never 类型
            // 无法产生一个可供返回的值，此时就会采用第一个分支返回的类型【u32】
            // never 强制转化为任意其他类型
            // 所以两个分支的返回其实是一样的
            // loop、painc! 都是 never 类型，因为不会返回内容
        };
    }

}