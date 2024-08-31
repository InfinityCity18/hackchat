use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

const PORT: u16 = 7312;
const SIGNATURE: &str = "github.com/InfinityCity18/hackchat";

pub enum OpCode {
    Message = 0,
    User = 1,
    Leave = 2,
}

pub fn udp_manager() -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT))?;
    socket.set_broadcast(true)?;
    socket.connect(SocketAddr::from((Ipv4Addr::BROADCAST, PORT)))?;

    Ok(())
}
