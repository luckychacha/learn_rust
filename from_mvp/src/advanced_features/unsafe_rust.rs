use std::slice;

// 匹配命名变量
// 隐藏着第二个语言，他没有强制内存安全保证：Unsafe Rust
//      - 和普通的 Rust 一样，但是有额外的超能力
// Unsafe Rust 存在的原因
//      - 静态分析是保守的
//          - 使用 Unsafe Rust：我知道自己的行为并承担相应风险
//      - 计算机硬件本身就是不安全的，Rust 需要能够进行底层系统编程
// 超能力
//      - 使用 unsafe 关键字来切换到 unsafe rust，开启一个块，里面放着 unsafe 代码
//      - 可以执行四个动作
//          - 解引用原始指针
//          - 调用 unsafe 函数或方法
//          - 访问或修改可变的静态变量
//          - 实现 unsafe trait
//      - 注意
//          - unsafe 并没有关闭借用检查或停用其他安全检查
//          - 任何内存安全相关的错误必须留在 unsafe 块里
//          - 尽可能隔离 unsafe 代码，最好将其封装在安全的抽象里，提供安全的 API
// 解引用原始指针【Raw Pointer】
//  - 可变的： *mut T
//  - 不可变的： *const T。意味着指针在解引用后不能直接对其进行赋值
//  - 注意：这里的 * 不是解引用符号，它是类型名的一部分。
//  - 与引用不同，原始指针：
//      - 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
//      - 无法保证能指向任何合理的内存
//      - 允许为 null
//      - 不识闲任何自动清理
//  - 放弃保证的安全，换区更好的性能与其他语言或硬件接口的能力
//  - 为什么要使用原始指针？
//      - 与 c 语言进行接口交互
//      - 构建借用检查器无法理解的安全抽象
// 调用 unsafe 函数或方法
//  - 调用前需要手动满足一些条件，因为 rust 无法对这些条件进行验证
//  - 需要在 unsafe 块里进行调用
//  - 函数包含 unsafe 代码不意味着需要将整个函数标记为 unsafe
//  - 将 unsafe 包裹在安全函数中是一种常见的操作
// 使用 extern 函数调用外部代码
//  - extern 关键字：简化创建和使用外部函数接口【FFI】的过程
//  - 外部函数接口【Foreign Function Interface】：它允许一种编程语言定义函数，并让其他编程语言调用这些函数
//  - 应用二进制接口【ABI：Application Binary Interface】：定义函数在汇编层的调用方式
//  - 从其他语言调用 Rust 函数
//      - 可以使用 extern 创建接口，其他语言通过他们可以调用 Rust 的函数
//      - 在 fn 前添加 extern 关键字，指定 ABI
//      - 还需添加 #[no_mangle] 注解：避免 Rust 在编译时改变它的名称
// 访问或修改一个可变静态变量
//  - Rust 支持全局变量，但因为所有权机制可能产生某些问题，例如数据竞争
//  - 在 Rust 里，全局变量叫做静态【static】变量
//  - 静态变量与常量类型
//  - 命名：全大写并且使用下划线连接
//  - 必须标注类型
//  - 静态变量只能存储 'static  生命周期的引用，无需显示标注
//  - 访问不可变的静态变量是安全的
// 常量和不可变静态变量的区别
//  - 静态变量：有固定的内存地址，使用它的值总会访问同样的数据
//  - 常量：允许使用他们的时候对数据进行复制
//  - 静态变量：可以使可变的，访问和修改静态可变变量是不安全的
// 实现不安全 trait
//  - 当某个 trait 中至少存在一个方法拥有编译器无法校验的不安全的因素时，就称这个 trait 是不安全的
//  - 声明 unsafe trait：在定义前加 unsafe 关键字
//      - 该 trait 只能在 unsafe 代码块中实现
// 何时使用 unsafe 代码
//  - 编译器无法保证内存安全，保证 unsafe 代码正确并不简单
//  - 有充足的理由使用 unsafe 代码时，就可以这么做
//  - 通过显示表姐 unsafe，可以在出现问题时轻松的定位

static HELLO: &str = "Hello!";
static mut COUNTER: u32 = 0;

unsafe trait Foo {}
unsafe impl Foo for i32 {}

fn add_to_count(inc: u32) {
    // 多进程时可能出现数据竞争
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        // pub unsafe fn from_raw_parts_mut
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn dereference_a_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // this operation is unsafe and requires an unsafe function or block
    // println!("r1: {}", *r1);

    unsafe {
        println!("r1: {}", *r1);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r: {}", *r);
    }

    // dangerous();
    unsafe {
        dangerous();
    }

    // extern demo
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}