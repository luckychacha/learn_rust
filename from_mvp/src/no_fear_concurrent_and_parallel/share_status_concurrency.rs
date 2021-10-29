use std::{sync::{Arc, Mutex}, thread};

// 共享状态的并发
// - Rust 支持用共享状态实现并发
// - Channel 类似于单所有权，一旦将值的所有权转移至 Channel，就无法使用它了
// - 共享内存类似于多所有权，多个线程可以同时访问同一内存
// 使用 Mutex 来每次只允许一个线程来访问数据
// - Mutex 是 mutual exclusion【互斥锁】的简写
// - 在同一时刻，mutext 只允许一个线程来访问某些数据
// - 想要访问数据
//      - 线程必须首先获取互斥锁【lock】，lock 数据结构是 mutex 的一部分，他能跟踪谁对数据又有独占访问权
//      - mutex 通常被描述为：通过锁定系统来保护它所持有的数据
// - Mutex 的两条规则
//      - 在使用数据之前，必须尝试获取锁【lock】。
//      - 使用完 mutex 所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁
// - Mutex<T> 的 API
//      - Mutex::new(数据) 来创建 Mutex<T>，Mutex<T> 是一个智能指针
//      - 访问数据，通过 lock 方法来获取锁
//          - 会阻塞当前线程
//          - lock 可能会失败
//          - 返回的是 MutexGuard 【智能指针，实现了 Deref 和 Drop】
// - 多线程共享 Mutex
//      - 使用 Arc<T> 来进行原子引用计数
//          - Arc 和 Rc 类似，他可以用于并发场景
//          - 为什么默认不使用 Act 呢？因为会牺牲性能
// - RefCell<T>/Rc<T> vs Mutex<T>/Arc<T>
//      - Mutex<T> 提供了内部可变性，和 Cell 家族一样
//      - 我们使用 RefCell<T> 来改变 Rc<T> 里面的内容
//      - 我们使用 Mutex<T> 来改变 Arc<T> 里面的内容
//      - 注意 Mutex<T> 有死锁的风险，例如 a 线程先锁 a 后锁 b，b 线程相反，二者就会死锁。

pub fn mutex_demo() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        // 第一个线程把 counter move 了，第二个线程 move 就会报错
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}