use std::net::UdpSocket;

fn main() {
    let server_addrs = "google.com";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(1)))
        .unwrap();

    let ping_msg = b"ping";

    for _ in 0..4 {
        let start = std::time::Instant::now();
        socket.send_to(ping_msg, server_addrs).unwrap();

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let elapsed = start.elapsed();
                println!("Received res from {}: {} in {:?}", addr, size, elapsed);
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
