use std::thread;

pub fn move_demo() {
    let v = vec![1, 2, 3];

    // 因为代码后面存在导致 v 的生命周期早于线程结束的代码，此时会报错。
    // value used here after move
    let handle = thread::spawn(move || {
        println!("here is a vector: {:?}", v);
    });

    
    // drop(v);

    handle.join().unwrap();
}