// Deref Trait
// - 实现 Deref Trait 使我们可以自定义解引用运算符 * 的行为。
// - 通过实现 Deref，智能指针可以像常规引用一样来处理。
// 解引用运算符
// - 常规引用也是一种指针
// 把 Box<T>当做解引用
// - Box<T>可以代替上例中的引用。
// 定义自己的智能指针
// Box<T> 被定义成拥有一个元素的 tuple struct
// 例子 MyBox<T>
// 实现 Deref Trait
// - 标准库中的 Deref Trait 要求我们实现一个 deref 方法：
// - 该方法借用 self，返回一个指向内部数据的引用
// 函数和方法的隐式解引用转化【Deref Coercion】
// - 隐式解引用转化【Deref Coercion】是为函数和方法提供的一种便捷特性
// - 假设 T 实现了 Deref trait：
//  - Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用
// - 当把某个类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配：
//  - Deref Coercion 就会自动发生
//  - 编译器会对 deref 进行一系列调用，来把它转换为所需的参数类型
//      - 在编译时就完成了，运行时没有额外的性能开销
// 解引用与可变性
// - 可使用 DerefMut trait 重载可变引用的 * 运算符
// - 在类型和 trait 在下列三种情况发生时：Rust 会执行 deref coercion：
//      1.当 T: Deref<Target=U>，允许 &T 转换为 &U
//      2.当 T: DerefMut<Target=U>，允许 &mut T 转换为 &mut U
//      3.当 T: Deref<Target=U>，允许 &mut T 转换为 &U【可变引用变为不可变引用】
use std::ops::Deref;

// MyBox 是个元组！
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

pub fn deref_trait_intro() {
    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 相当于 *(z.deref())
    assert_eq!(5, *z);


    let a = MyBox::new(String::from("Rust"));

    // &a = &MyBox<String>，MyBox 实现了 Deref Trait
    // deref &String， String 也实现了 Deref Trait，也可以 deref
    // 最后得到 &str
    hello(&a);
}