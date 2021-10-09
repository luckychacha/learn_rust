// 并发
// - Concurrent：程序的不同部分之间独立的执行，可能叫并发
// - Parrallel：程序的不同部分同时运行，可能叫并行
// - Rust 无畏并发：允许你编写没有细微 Bug 的代码，并在不引入新 Bug 的情况下易于重构
// - 注意：并发泛指：Concurrent 和 Parrallel。

// 16.1 使用线程同时运行代码
// - 进程与线程
//      - 大部分 OS 里，代码运行在进程【process】中，OS 同时管理多个进程。
//      - 在你的程序里，各种部分可以同时运行，运行这些独立部分就是线程【thread】
//      - 多线程运行：
//          - 提升性能表现
//          - 增加复杂性：无法保障各线程的执行顺序
// - 多线程可导致的问题
//      - 竞争状态，线程以不一致的顺序访问数据或资源
//      - 死锁，两个线程彼此等待对方使用完所有的资源，线程无法复制继续
//      - 只有在某些情况下发生的 Bug，很难可靠地复制现象和修复
// - 实现线程的方式
//      - 通过调用 OS 的 API 来创建线程，1：1 模型
//          - 需要较小的运行时
//      - 语言自己实现的线程【绿色线程】，M：N 模型
//          - 需要较大的运行时
//      - Rust：需要权衡运行时的支持，默认支持 1：1 模型
// - 通过 spawn 创建新线程
//      - 通过 thread::spawn 函数可以创建新线程
//          - 参数：一个闭包【在新线程里运行的代码】
// - 通过 join Handle 来等待所有线程的完成
//      - 调用其 join 方法，可以等待对应的其他线程的完成
// - join 方法：调用 handle 的 join 方法会组织当前运行线程的执行，知道 handle 所有便是的这些线程终结
// - 使用 move 闭包
//      - move 闭包通常和 thread::spawn 函数一起使用，它允许你使用其他线程的数据
//      - 创建线程时，把值的所有权从一个线程转移到另一个线程