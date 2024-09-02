use std::io::{Read, Write};
use std::net::TcpStream;

fn read_response(stream: &mut TcpStream) {
    let mut buf = [0; 1024];
    let size = stream.read(&mut buf).unwrap();
    println!("RES: {}", std::str::from_utf8(&buf[..size]).unwrap());
}

fn main() {
    let addr = "localhost:1025";
    let mut stream = TcpStream::connect(addr).expect("connect to the socket");
    read_response(&mut stream);

    stream
        .write_all(b"HELO localhost\r\n")
        .expect("To say hello");
    read_response(&mut stream);

    stream
        .write_all(b"MAIL FROM:<hello@hello.com>\r\n")
        .expect("to say from");
    read_response(&mut stream);

    stream
        .write_all(b"RCPT TO:<mama@mama.io>\r\n")
        .expect("to set recept");
    read_response(&mut stream);

    stream.write_all(b"DATA\r\n").expect("to set data");
    read_response(&mut stream);

    stream
        .write_all(b"From: hello@hello.com\r\nTo: mama@mama.io\r\nSubject: Sub\r\n\r\nWe are the data content stuff....\r\n.\r\n")
        .expect("to send data");
    read_response(&mut stream);

    stream.write_all(b"QUIT\r\n").expect("to set quit");
    read_response(&mut stream);

    println!("Email done");
}
