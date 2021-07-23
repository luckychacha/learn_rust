use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time;


fn handle_client(mut tcp_stream: TcpStream) ->std::io::Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = tcp_stream.read(&mut buf)?;
        let message = match String::from_utf8(Vec::from(&buf[..bytes_read])) {
            Ok(line) => {
                println!("receive info: {}", line);
                line
            }
            Err(_) => {
                println!("read stream error, raw content: {:?}", &buf[ ..bytes_read ]);
                "".to_string()
            }
        };
        if "quit\n".eq(&message) {
            println!("???");
            break;
        }

        match tcp_stream.write(&buf[ ..bytes_read]) {
            Ok(_) => {
                println!("write echo success");
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        std::thread::sleep(time::Duration::from_secs(1_u64));
    }

    Ok(())
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666")?;

    let mut thread_vec: Vec<std::thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        let handle = std::thread::spawn(move || {
            let _res: () = handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }


    Ok(())
}
