use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

fn main() -> std::io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:6666")?;
    for _ in 0..10 {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("fail to read from stdin");

        stream.write(input.as_bytes())
            .expect("fail to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_until(b'\n', &mut buffer)
            .expect("unable to read from stream");

        print!(
            "{}",
            std::str::from_utf8(&buffer)
                .expect("Could not write buffer as string")
        );

        println!("====");

    }
    Ok(())
}
