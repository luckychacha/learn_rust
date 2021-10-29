// 函数指针
//  - 将函数传递给其他函数
//  - 函数在传递过程中会被强制转换为 fn 类型
//  - fn 类型就是“函数指针【function pointer】”
// 函数指针与闭包的不用
//  - fn 是一个类型，不是一个 trait
//      - 可以直接指定 fn 为参数类型，不用声明一个以 Fn trait 为约束的泛型参数
//  - 函数指针实现了全部 3 种闭包 trait【Fn，FnMut，FnOnce】：
//      - 总是可以把函数指针用作参数传递给一个接受闭包的函数
//      - 所以倾向于搭配闭包 trait 的泛型来编写函数：可以同时接收闭包和普通函数
//  - 某些情景，指向接收 fn 而不接收闭包：
//      - 与外部不支持闭包的代码交互：C 函数
// 返回闭包
//  - 闭包使用 trait 进行表达，无法在函数中直接返回一个闭包，
//      可以将一个实现了该 trait 的具体类型作为返回值

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn advanced_function_and_closures_demo() {
    let res = do_twice(add_one, 5);
    println!("The answer is: {}", res);

    let list_of_numbers = vec![1,2,3,4];
    // 闭包
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();

    // 传入函数
    let list_of_numbers = vec![1,2,3,4];
    let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();

    enum Status {
        Value(u32),
        Stop,
    }
    let v = Status::Value(3);

    let list_of_statuses: Vec<Status> = (0u32..20)
    .map(Status::Value)
    .collect();
}