// Rc<T> 引用计数智能指针
//  - 有时，一个值会有多个所有者
//      - 为支持多重所有权：Rc<T>
//      - reference counting【引用计数】
//      - 追追踪所有到这个值的引用
//      - 0 个引用，该值可以被清理掉
// - 使用场景
//      - 需要在 heap 上分配数据，这些数据被程序的多个部分读取【只读】，但在编译时无法确定哪个部分最后使用完这些数据
//      - Rc<T> 只能在单线程中使用
// - 例子
//      - Rc<T> 不在与导入模块【prelude】
//      - Rc::clone(&a) 函数：增加引用计数
//      - Rc::strong_count(&a)：获得引用计数【强引用】
//      - Rc::weak_count(&a)：获得引用计数【弱引用】
// - Rc::clone(&a) 不会深度拷贝，速度比 a.clone() 快
// - Rc<T> 通过不可变引用，使你可以在程序不同部分之间共享只读数据
// - 但是如何允许数据变化呢？ 使用 RefCell。

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::smart_pointers::rc_intro::List::{Cons, Nil};
use std::rc::Rc;

pub fn rc_intro() {
    // 5->10
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // count after creating a = 1

    // Rc::clone(&a) 不会深度拷贝，速度比 a.clone() 快
    // 3->5->10
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // count after creating b = 2

    // 4->5->10
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // count after creating c = 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // count after c goes out of scope = 2
}