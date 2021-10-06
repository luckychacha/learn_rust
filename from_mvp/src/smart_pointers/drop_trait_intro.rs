// Drop Trait
//  - 实现 Drop Trait，可以让我们自定义当值离开作用域时发生的动作
//      - 例如：文件、网络资源释放等。
//      - 任何类型都可以实现 Drop Trait
//  - Drop trait 只要求实现 drop 方法
//      - 参数：对 self 的可变引用
//  - Drop trait 在预导入模块里【prelude】
// 使用 std::mem::drop 来提前 drop 值
// - 很难直接禁用自动的 crop 功能，也没必要
//      - Drop trait 的目的就是进行自动的释放处理逻辑
// - Rust 不允许手动调用 Drop trait 的 drop 方法

struct CustomSamrtPointer {
    data: String,
}

impl Drop for CustomSamrtPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSamrtPointer with data: `{}`", self.data);
    }
}

pub fn drop_intro() {
    let _c = CustomSamrtPointer { data: String::from("my stuff") };
    drop(_c);
    let _d = CustomSamrtPointer { data: String::from("other stuff") };

    println!("CustomSamrtPointer created");
}