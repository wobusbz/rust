use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_clinet(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    
    println!("收到一些内容: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() ->std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9901") ?;
    
    for stream in listener.incoming(){
        handle_clinet(stream?);
    }
    Ok(())
}
