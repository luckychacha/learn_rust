use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use hello::ThreadPool;

// 构建多线程 Web 服务器
//  - 在 socket 上监听 TCP 连接
//  - 解析少量的 HTTP 请求
//  - 创建一个合适的 HTTP 响应
//  - 使用线程池改进吞吐量

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //  take 2 让程序只执行 2 次。
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}