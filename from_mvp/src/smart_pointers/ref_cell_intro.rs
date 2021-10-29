
// RefCell<T> 和内部可变性
// 内部可变性
// - 内部可变性是 Rust 的设计模式之一
// - 它允许你在只持有不可变引用的前提下对数据进行修改
//      - 数据结构中使用了 unsafe 代码来绕过 Rust 正常的可变性和借用规则
// RefCell<T>
// - 与 Rc<T> 不同，RefCell<T> 类型代表了其持有数据的唯一所有权。
// - 回忆：借用规则：在任何给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用。引用总是有效的。
// RefCell<T> 和 Box<T> 的区别
// - Box<T> 是编译阶段强制代码遵守借用规则，否则出现错误
// - RefCell<T> 只会在运行时检查借用规则，否则触发 panic
// 借用规则在不用阶段进行检查的比较
// - 编译阶段：尽早暴露问题，没有运行时开销，对大多数场景是组价选择，Rust 默认行为
// - 运行时：问题暴露延后，甚至到生产环境。因借用计数产生些许性能损失，实现某些特定的内存安全场景。【不可变环境中修改自身数据】
// RefCell 只能用于单线程的场景。
// 使用 Box<t>、Rec<T>、RefCell<T> 的依据
//                   Box<t>             Rec<T>          RefCell<T>
// 同一数据的所有者     一个                多个                一个
// 可变性、借用检查     可变、不可变借用      不可变借用        可变、不可变借用
//                  【编译时检查】         【编译时检查】    【运行时检查】
// 内部可变性：可变的借用一个不可变的值
// 使用 RefCell<T> 在运行时记录借用的信息
// - 两个方法【安全接口】
//      - borrow 方法：返回智能指针 Ref<T>，实现了 Deref
//      - borrow_mut 方法：返回智能指针 RefMut<T>，实现了 Deref
// - RefCell<T> 会记录当前存在多少个活跃的 Ref<T> 和 RefMut<T> 智能指针：
//      - 每次调用 borrow，不可变借用计数加1
//      - 任何一个 Ref<T> 的值离开作用域被释放时，不可变借用计数减少1
//      - 每次调用 borrow_mut，可变借用计数加1
//      - 任何一个 RefMut<T> 的值离开作用域被释放时，可变借用计数减1
// - 以此计数来维护借用检查规则：
//      - 任何一个给定时间里，只允许拥有多个不可变借用或一个可变借用。
// - 将 Rc<T> 和 RefCell<T> 结合使用来实现一个拥有多重所有权的可变数据
//      - Rc<T> 允许多个所有者持有同一个数据，但是只能提供对数据的不可变访问。
//      - 所以在 Rc<T> 中存储 RefCell<T>，那么就可以定义出有用多个所有者而且能够进行修改的值了。
// 其他可实现内部可变性的类型
//  - Cell<T> 通过复制来访问数据
//  - Mutext<T> 用于实现跨线程情形下的内部可变性模式

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::smart_pointers::ref_cell_intro::List::{Cons, Nil};


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger, 
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota");
        }
    }
}

pub fn ref_cell_concept() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(
        Cons(
            Rc::clone(&value),
            Rc::new(Nil)
        )
    );
    let b = Cons(
        Rc::new(RefCell::new(6)),
        Rc::clone(&a)
    );
    let c = Cons(
        Rc::new(RefCell::new(7)),
        Rc::clone(&a)
    );

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("a after = {:?}", b);
    println!("a after = {:?}", c);
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}