
// Rust 的内存安全机制可以保证很难发生内存泄漏，但不是不可能
// 例如使用 Rc<T> 和 RefCell<T> 就可能创造出循环引用，从而发生内存泄漏
// - 每个项的引用数量不会变成 0，值也不会被处理掉。
// 防止内存泄漏的办法
// - 开发时注意
// - 重新组织数据结构：溢写引用老表所有权，一些引用不表达所有权
//      - 循环引用中的一部分具有所有权关系，另一部分不涉及所有权关系
//      - 而只有所有权关系才影响值的清理
// 防止循环引用可以把 Rc<T> 换成 Weak<T>
// Rc::clone 为 Rc<T> 实例的 stron_count 加 1，Rc<T> 的实例只有在 strong_count 为 0 的时候才会被清理
// Rc<T> 实例可以通过调用 Rc::downgrade 方法可以创建值的 Weak Reference【弱引用】。
//      - 返回类型是 Weak<T>【智能指针】
//      - 调用 Rc::downgrade 会为 weak_count 加 1
// Rc<T> 使用 weak_count 来追踪存在多少 Weak<T>。
// weak_count 不为 0 并不影响 Rc<T> 实例的清理。
// Strong vs Weak
// Strong Reference 是关于如何分享 Rc<T> 实例的所有权
// Weak Reference 并不表达上述意思
// 使用 Weak Reference 并不会创建循环引用：
//      - 当 Strong Reference 数量为 0 的时候，Weak Reference 会自动断开
// 在使用 Weak<T> 前，需保证它指向的值仍然存在：
//      - 在 Weak<T> 实例上调用 upgrade 方法，返回 Option<Rc<T>>

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use std::borrow::Borrow;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::smart_pointers::cycle_reference_cause_overflow::List::{Cons, Nil};

pub fn cycle_reference_cause_overflow() {
    let a = Rc::new(
        Cons(
            5,
            RefCell::new(Rc::new(Nil))
        )
    );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(
        Cons(
            10,
            RefCell::new(Rc::clone(&a))
        )
    );
    println!("a rc count after b initial = {}", Rc::strong_count(&a));
    println!("b initial rc count = {:?}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after charging a = {}", Rc::strong_count(&b));
    println!("a rc count after charging a = {}", Rc::strong_count(&a));

    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // println!("a next item = {:?}", a.tail());
    
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    // leaf.parent.borrow() 得到 leaf 的 parent 属性的不可变引用
    // 使用 upgrade 方法从 Weak<T> 变成 Rc<T>
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // branch strong = 1, weak = 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        // leaf strong = 2, weak = 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    // leaf parent = None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}