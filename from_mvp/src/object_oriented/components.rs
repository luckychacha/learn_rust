// Trait 对象执行的是动态派发
// - 将 trait 约束作用于泛型时，Rust 编译器会执行单态化：
//      - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
// - 通过单态化生成的代码会执行静态派发【stratic dispatch】，在编译过程中确定调用的具体方法。
// - 动态派发【dynamic dispatch】
//      - 无法在编译过程中确定你调用的究竟是哪一种方法
//      - 编译器会产生额外的代码以便在运行时找出希望调用的方法
// - 使用 trait 对象，会执行动态派发
//      - 产生运行时开销
//      - 组织编译器内联方法代码，使得部分优化操作无法进行
// Trait 对象必须保证对象安全
// - 只能满足对象安全【object safe】的 trait 转化为 trait 对象
// - Rust 采用一系列规则来判定某个对象是否安全，只需记住两条：
//      - 方法的返回类型不是 Self
//      - 方法中不包含任何泛型类型参数
//      - 【例如 Clone 这个 trait】就不符合对象安全，返回的是 Self
//      - 所以无法使用 Box<dyn Clone> 作为对象
// 实现面向对象的设计模式
// - 状态模式
//      - 一个值拥有的内部状态由数个状态对象【state object】表达而成
//      - 而值的行为则随着内部状态的改变而改变
// - 使用状态模式意味着：
//      - 业务需求变化时，不需要修改持有状态的值的代码，或者使用这个值的代码
//      - 只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象
//      - 例子：发布博客的流程：
//          1.新建【new】博文时，生成空白的草稿文档。
//          2.草稿编写【add_text】完成之后可以请求进行审批【request_review】
//          3.审批通过【approve】之后就可以正式对外发布【content】，未通过审批的文章不能发布
// - 状态模式的取舍权衡
//      - 缺点：某些状态存在耦合、需要重复实现逻辑代码。
// - 将状态和行为编码为类型
//      - Rust 类型检查系统会通过编译时错误来组织用户使用无效的代码

pub trait Draw {
    fn draw(&self);
}

// 使用 Box 更灵活，如果用泛型的话，只能支持一种实现类型，
// 而使用 Box 就可以表达只要实现了 Draw 这个 trait 的结构体都可以放在 Vec 中
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "button width: {}, height: {}, label: {}",
            self.width,
            self.height,
            self.label
        );
    }
}