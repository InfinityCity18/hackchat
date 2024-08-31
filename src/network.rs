use std::net::UdpSocket;

fn udp_manager(addr: &str) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(addr)?;
    socket.set_broadcast(true)?;
    Ok(())
}
