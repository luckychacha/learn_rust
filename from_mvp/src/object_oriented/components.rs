// Trait 对象执行的是动态派发
// - 将 trait 约束作用于泛型时，Rust 编译器会执行单态化：
//      - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
// - 通过单态化生成的代码会执行静态派发【stratic dispatch】，在编译过程中确定调用的具体方法。
// - 动态派发【dynamic dispatch】
//      - 无法在编译过程中确定你调用的究竟是哪一种方法
//      - 编译器会产生额外的代码以便在运行时找出希望调用的方法

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