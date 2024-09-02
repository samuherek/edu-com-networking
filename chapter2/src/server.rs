use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4433").unwrap();
    println!("Listening on: {:?}", socket);

    let mut buf = [0; 1024];
    loop {
        let (size, addr) = socket.recv_from(&mut buf).unwrap();
        println!(
            "Received: {} of {} from {}",
            String::from_utf8(buf.to_vec()).unwrap(),
            size,
            addr
        );
        socket.send_to(&buf.to_vec(), addr).unwrap();
    }
}
