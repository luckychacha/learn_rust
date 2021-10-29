use std::{fmt, ops::Add};

// 在 Trait 定义中使用关联类型来指定站位类型
//  - 关联类型【associated type】是 Trait 中的 类型占位符，它可以用于 Trait 的方法签名中：
//      - 可以定义出包含某些类型的 Trait，而在实现前无需知道这些类型是什么
// 关联类型和泛型的区别
// 泛型                                         关联类型
// 1.每次实现 Trait 时标注类型              1.无需标注类型
// 2.可以为一个类型多次实现某个 Trait        2.无法为单个类型多次实现某个 Trait
//   【不用的泛型参数，换一个泛型参数就可
//      以添加一种实现】
// 默认泛型参数和运算符重载
//  - 可以在使用泛型参数时为泛型指定一个默认的具体类型
//  - 语法：<PlaceholderType=ConcreteType>
//  - 这种技术常用于运算符重载【operator overloading】
//  - Rust 不允许创建自己的运算符及重载任意的运算符
//  - 但可以通过实现 std::ops 中列出的那些 trait 来重载一部分相应的运算符
// 默认泛型参数的主要应用场景
//  - 扩展一个类型而不破坏现有代码
//  - 允许在大部分用户都不需要的特定场景下自定义
// 完全限定语法【Fully Qualified Syntax】
//  - 如何调用同名语法
//  - 调用方式：<Type as Trait>::function(receiver_if_method, next_arg, ...)
//      - 可以在任何调用函数或方法的地方调用
//      - 允许忽略那些从其他上下文能推倒出来的部分
//      - 当 Rust 无法区分你期望调用哪个具体实现的时候，才需要这种语法。
// 使用 newtype 模式在外部类型上实现外部 trait
//  - 孤儿规则：只有当 trait 或类型定义在本地包时，才能为该类型实现这个 trait
//  - 可以通过 newtype 模式来绕过这一规则
//      - 利用 tuple struct【元组结构体】创建一个新的类型

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
    }
}

struct Counter {}

// 此时会报错因为 OutlinePrint 需要先实现 Display
// `Counter` doesn't implement `std::fmt::Display`
// impl OutlinePrint for Counter {

// }

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {

    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 为毫米【Millimeters】 这个结构体实现一个和米【Meters】结构体相加【Add】这个 trait
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(rhs.0 * 1000 + self.0)
    }
}

trait Wizard {
    fn fly(&self);
    fn run();
}

trait Pilot {
    fn fly(&self);
    fn run();
}

struct Human;

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard fly");
    }
    fn run() {
        println!("Wizard run");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot fly");
    }
    fn run() {
        println!("Pilot run");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human fly");
    }
    fn run() {
        println!("Human run");
    }
}

// Vec 这个类型不在本地包，Display 这个 trait 也不在本地包，
// 但是 Wrapper 在本地的，此时就是可以为 Wrapper 实现 Display
// Wrapper.0 就是 Vec
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn advanced_traits_demo() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let human = Human;
    // 本身的 fly 方法
    human.fly();
    // trait 可以被多个类型实现，但是通过参数传入，就知道是哪个具体的实现了
    // 如果 trait 的方法和自身的方法没有参数，这时候就需要使用完全限定语法
    Pilot::fly(&human);
    Wizard::fly(&human);
    <Human as Pilot>::run();
    <Human as Wizard>::run();
}