use std::{thread, time::Duration};

pub fn thread_spawn_init() {
    // 创建一个子线程并运行
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawn thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

   let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程继续执行，主线程结束之后，离开作用域时，子线程会直接结束。
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join 会阻塞主线程的执行，直到 handle 所对应的线程的结束
    // 如果把这行代码放在主线程的逻辑之前，那么输出就会变成：
    // hi number 1 from the spawn thread!
    // hi number 2 from the spawn thread!
    // hi number 3 from the spawn thread!
    // hi number 4 from the spawn thread!
    // hi number 5 from the spawn thread!
    // hi number 6 from the spawn thread!
    // hi number 7 from the spawn thread!
    // hi number 8 from the spawn thread!
    // hi number 9 from the spawn thread!
    // hi number 1 from the main thread
    // hi number 2 from the main thread
    // hi number 3 from the main thread
    // hi number 4 from the main thread
    // 即 handle 的逻辑都执行完了，才往下执行。
    handle.join().unwrap();
}