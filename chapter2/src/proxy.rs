use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn get_host(data: &str) -> Option<String> {
    for line in data.lines() {
        if line.starts_with("Host: ") {
            let val = line.split_whitespace().nth(1).expect("to split the host");
            if val.contains(":") {
                return Some(val.to_string());
            } else {
                return Some(format!("{}:80", val));
            }
        }
    }
    None
}

fn main() {
    let conn = TcpListener::bind("127.0.0.1:8222").expect("To bind listener");
    println!("Listening on 127.0.0.1:8222");

    for stream in conn.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(move || {
                    let mut buf = [0; 4096];
                    let bytes = stream.read(&mut buf).expect("to read the data");
                    if bytes == 0 {
                        return;
                    }
                    let data = std::str::from_utf8(&buf[..bytes]).expect("to convert to string");
                    println!("the req:\n {}", data);
                    let host = match get_host(&data) {
                        Some(h) => h,
                        None => {
                            eprintln!("Could not find hsot");
                            return;
                        }
                    };
                    println!("we have host: {:?}", host);

                    let mut conn = TcpStream::connect(host).expect("to connect to the end host");
                    conn.write_all(&buf[..bytes]).expect("to send the bytes");
                    let mut res = [0; 4096];
                    let res_bytes = conn.read(&mut res).expect("to read res bytes");
                    println!("res: {}", std::str::from_utf8(&res[..res_bytes]).expect("to convert res"));
                    stream
                        .write_all(&res[..res_bytes])
                        .expect("to send the bytes back");
                });
            }
            Err(e) => {
                println!("ERROR: failed the conn {}", e);
            }
        }
    }

    println!("proxy");
}
