// 模式
// - 模式是 Rust 中的一种特殊语法，用于匹配复杂和简单类型的结构
// - 将模式与匹配表达式和其他构造结合使用，可以更好的控制程序的控制流
// - 模式由一下元素组成：
//      - 字面值
//      - 结构的数组、enum、、struct、tuple
//      - 变量
//      - 通配符
//      - 占位符
// - 想要使用模式，需要将其与某个值进行比较：
//      - 如果模式匹配，就可以在代码中使用这个值的相应部分
// match 的 Arm
// match 表达式的要求：
//      - 详尽【包含所有的可能性】
//      - 一个特殊的模式：_【下划线】
//      - 他会匹配任何东西
//      - 不会绑定到变量
//      - 通常用于 match 的最后一个 arm：或用于忽略某些值
// if let 表达式
//      - 作为一种剪短的方式来等价的代替只有一个匹配项的 match
//      - 可选的可以拥有 else，包括：
//          - else if
//          - else if let
//      - 但，if let 不会检查穷尽性
pub fn patter_demo() {
    let color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is tuesday!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blus as the background color");
    }
}