use color_eyre::Result;
use std::{
    net::{Ipv6Addr, SocketAddrV6, UdpSocket},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

#[tokio::main]
async fn main() -> Result<()> {
    let multicast_addr =
        SocketAddrV6::new(Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 0x100), 34000, 0, 0);

    let local: SocketAddrV6 = "[::]:34000".parse::<SocketAddrV6>().unwrap();

    let socket = UdpSocket::bind(local).expect("Couldn't bind to localhost:34000");
    println!("Socket bound");

    // socket.set_read_timeout(Some(Duration::new(1, 0))).unwrap();

    socket
        .join_multicast_v6(
            &"ff02::100".parse().expect("Multicast address was invalid"),
            0,
        )
        .expect("Unable to join multicast group");
    println!("Joined multicast group");

    socket
        .set_multicast_loop_v6(false)
        .expect("Unable to set multicast loop");

    let send_socket = Arc::new(socket);
    let receive_socket = send_socket.clone();

    tokio::spawn(async move {
        loop {
            send_socket
                .send_to(b"testing", multicast_addr)
                .expect("Unable to send");
            println!("Sent message");
            sleep(Duration::new(1, 0));
        }
    });

    loop {
        let mut buf = [0u8; 1400];

        while let Ok((number_of_bytes, src_addr)) = receive_socket.recv_from(&mut buf) {
            let bytes = &buf[..number_of_bytes];
            println!("{:?}: {:?}", src_addr, String::from_utf8(bytes.to_vec()));
        }
    }
}
