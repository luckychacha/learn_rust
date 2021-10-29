// Box<T>
//  Box<T>是最简单的智能指针：
//  - 允许你在 heap 上存储数据【而不是 stack】
//  - stack 上指向 heap 数据的指针
//  - 没有性能开销
//  - 没有其他额外的功能
//  - 实现了 Deref trait 和 Drop trait
//  Box<T> 的常用场景
//  - 在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小
//  - 当你有大量数据时，想移交所有权，但需要确保在操作时数据不会被复制。
//  - 使用某个值的时候，你只关心它是否实现了特定的 trait，而不关心它的具体类型。
// 使用 Box 赋能递归类型
//  - 在编译时，Rust 需要知道一个类型所占的空间大小
//  - 而递归类型的大小无法在编译时确定。
//  - 但 Box 类型可以解决【Cons List】
//  关于 Cons List
//  - Cons List 是来自 Lisp 语言的一种数据结构
//  - Cons List 里每个成员由两个元素组成。
//    - 当前项的元素 和 下一个元素
//  - Cons List 里最后一个成员只包含一个 Nil 值，没有下一个元素。
//  - Rust 通常情况下使用 Vec
//  - Rust 如何确定为枚举分配的空间大小
//    - enum Message {
//    -     Quit,
//    -     Move(x: i32, y: i32),
//    -     Write(String),
//    -     ChangeColor(i32, i32, i32),
//    - )
//    - enum Message {}
//    - enum Message {}
use crate::smart_pointers::box_concept::List::{Cons, Nil};

pub fn box_concept() {
    // 存储在 stack 上
    let a = 5;
    // 存储在 heap 上
    let b = Box::new(5);
    println!("a = {}", a);
    println!("b = {}", b);
    let _list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}