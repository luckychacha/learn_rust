use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time;


fn handle_client(mut tcp_stream: TcpStream) ->std::io::Result<()> {
    // 创建 buffer
    let mut buf = [0; 512];
    // 循环开始
    loop {
        // 读取流中的数据
        let bytes_read = tcp_stream.read(&mut buf)?;
        // 将 bytes 转为 String，并且判断是否转换成功
        let message = match String::from_utf8(Vec::from(&buf[..bytes_read])) {
            Ok(line) => {
                print!("receive info: {}", line);
                line
            }
            Err(_) => {
                print!("read stream error, raw content: {:?}", &buf[ ..bytes_read ]);
                "".to_string()
            }
        };
        // 如果匹配到 quit 或者 空消息 就停止循环
        if "quit\n".eq(&message) || "\n".eq(&message) {
            break;
        }

        // 将接收到的消息作为返回，并且发现错误的话就打印错误
        match tcp_stream.write(&buf[ ..bytes_read]) {
            Ok(_) => {
                print!("write echo success: {}", message);
            }
            Err(e) => {
                print!("{:?}", e);
            }
        }
        // 线程停 1 秒
        std::thread::sleep(time::Duration::from_secs(1_u64));
    }

    // 返回 Ok
    Ok(())
}


fn main() -> std::io::Result<()> {
    // 创建一个 TcpListener，由于可能创建失败，但是这个属于可恢复的错误，所以用 ?
    let listener = TcpListener::bind("127.0.0.1:6666")?;

    // 创建一个向量用于收集 JoinHandle
    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

    // 读取连接
    for stream in listener.incoming() {
        // 检查连接返回是否为 Ok()，如果不是就 panic
        let stream = stream.expect("failed");
        // 创建线程
        let handle = std::thread::spawn(move || {
            // 调用 handle_client 函数处理数据，并且对错误进行处理
            let _res: () = handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        // 收集 JoinHandler
        thread_vec.push(handle);
    }

    // 把所有 JoinHandler 执行 join 操作，保证主线程等待子线程结束之后才结束。
    for handle in thread_vec {
        handle.join().unwrap();
    }

    // 返回 Ok
    Ok(())
}
