use color_eyre::Result;
use std::{
    net::{Ipv6Addr, SocketAddrV6, UdpSocket},
    thread::sleep,
    time::Duration,
};

fn main() -> Result<()> {
    println!("Hello, world!");

    let socket = UdpSocket::bind("[::1]:34000").expect("Couldn't bind to localhost:34000");

    socket
        .join_multicast_v6(
            &"ff01::100".parse().expect("Multicast address was invalid"),
            0,
        )
        .expect("Unable to join multicast group");

    let multicast_addr =
        SocketAddrV6::new(Ipv6Addr::new(65281, 0, 0, 0, 0, 0, 0, 256), 34001, 0, 0);

    loop {
        socket
            .send_to(b"testing", multicast_addr)
            .expect("Unable to send");
        sleep(Duration::new(1, 0));
    }
}
