use std::{sync::mpsc, thread, time::Duration};

// 消息传递
//  - 一种很流行且能保证安全并发的技术就是：消息传递。
//  - 线程【或 Actor】通过彼此发送消息【数据】来进行通信
// Go 语言的名言：不要用共享内存来通信，要用通信来共享内存。
// Rust：Channel【标准库提供】
//  - Channel 包含：发送端、接收端
//  - 调用发送端的方法，发送数据
//  - 接收端会检查和接受到达的数据
//  - 如果发送端、接收端任意一端被丢弃了，那么 Channel 就关闭了
// 创建 Channel
//  - 使用 mpsc::channel 函数创建 Channel
//  - 返回一个 tuple【元组】，里面的元素分别是发送端、接收端
// 发送端的 send 方法
//  - 参数：想要发送的数据
//  - 返回：Result<T, E>
//      - 如果有问题【例如接收端被丢弃】，就返回一个错误
// 接收点的方法
//  - recv 方法：阻止当前线程执行，直到 Channel 中有值被送来
//      - 一旦有值收到，就返回 Result<T, E>
//      - 当发送端关闭，就会收到一个错误
//  - try_recv 方法，不会阻塞
//      - 立即返回 Result<T, E>
//          - 有数据到达，返回 Ok，里面包含着数据
//          - 否则，返回错误
//      - 通常会使用循环调用来检查 try_recv 的结果
// Channel 和所有权转移
//  - 所有权在消息传递中非常重要：能帮你编写安全、并发的代码
// 通过克隆创建多个发送者


pub fn channel_demo() {
    let (sender, reveiver) = mpsc::channel();
    // 单个值的场景
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     sender.send(val).unwrap();

    //     // println!("val is {}", val);
    // });

    // // recv 会一直阻塞线程直到有消息被传入为止。
    // let received = reveiver.recv().unwrap();
    // println!("Got: {}", received);

    // 多个值的场景
    let sender_c = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let vals = vec![
            String::from("clone: hi"),
            String::from("clone: from"),
            String::from("clone: the"),
            String::from("clone: thread"),
        ];

        for val in vals {
            sender_c.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in reveiver {
        println!("Got: {}", received);
    }
}
