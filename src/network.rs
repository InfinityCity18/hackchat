use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    sync::{mpsc::Receiver, Arc, Mutex},
    time::{Duration, Instant},
};

use simple_crypt::{decrypt, encrypt};

const PORT: u16 = 7312;
const SIGNATURE: &str = "github.com/InfinityCity18/hackchat";
const PRESENCE_TIMER: Duration = Duration::from_secs(10);

pub enum Op {
    Message(OpCode, String, String),
    User(OpCode, String),
    Leave(OpCode, String),
}

pub struct Arcs {
    pub users: Arc<Mutex<HashSet<String>>>,
    pub network_messages: Arc<Mutex<Vec<(String, String)>>>,
    pub chat_messages: Arc<Mutex<(usize, Vec<String>)>>,
}

pub enum OpCode {
    Message = 0,
    User = 1,
    Leave = 2,
}

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Message),
            1 => Ok(Self::User),
            2 => Ok(Self::Leave),
            _ => Err(()),
        }
    }
}

pub fn udp_manager(rx: Receiver<Op>, room: String, arcs: Arcs) -> Result<(), std::io::Error> {
    let socket = Arc::new(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT))?);
    socket.set_broadcast(true)?;
    socket.connect(SocketAddr::from((
        "10.21.37.255".parse::<Ipv4Addr>().unwrap(),
        PORT,
    )))?;

    let presence_map = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));

    let mut read_buf: Vec<u8> = [0; 65536].to_vec();

    let presence_map_clone = presence_map.clone();
    let users_clone = arcs.users.clone();
    let room_clone = room.clone();

    std::thread::spawn(|| presence_manager(presence_map_clone, users_clone));
    std::thread::spawn({
        let socket = socket.clone();
        move || udp_sender(socket, rx, room_clone)
    });

    loop {
        let amount_read = socket.recv(&mut read_buf)?;

        if let Some(sign) = read_buf.get(0..SIGNATURE.len()) {
            if sign != SIGNATURE.as_bytes() {
                continue;
            }
        } else {
            continue;
        }

        if let Some(payload) = read_buf.get(SIGNATURE.len()..amount_read - SIGNATURE.len()) {
            if let Ok(decrypted) = decrypt(payload, room.as_bytes()) {
                if let Some(v) = read_buf.get(0) {
                    if let Ok(opcode) = OpCode::try_from(*v) {
                        match opcode {
                            OpCode::Message => {
                                let mut username = Vec::new();
                                let mut msg = Vec::new();
                                let mut username_read = false;
                                for c in decrypted {
                                    if c == 0 {
                                        username_read = true;
                                        continue;
                                    }
                                    if username_read {
                                        msg.push(c);
                                    } else {
                                        username.push(c);
                                    }
                                }
                                let username = match String::from_utf8(username) {
                                    Ok(s) => s,
                                    Err(_) => continue,
                                };
                                let msg = match String::from_utf8(msg) {
                                    Ok(s) => s,
                                    Err(_) => continue,
                                };

                                arcs.network_messages
                                    .lock()
                                    .unwrap()
                                    .push((username.clone(), msg.clone()));
                                let mut lock = arcs.chat_messages.lock().unwrap();
                                let mut lines = Vec::new();
                                let mut i = lock.1.len() + 1;
                                let formated_msg = format!("|{username}| {msg}");

                                let mut line = String::new();
                                let mut m = 0;
                                for c in formated_msg.chars() {
                                    if m % lock.0 == 0 {
                                        if line.len() > 0 {
                                            lines.push(line.clone());
                                        }
                                        line.clear();
                                        line += " ";
                                        line += &i.to_string();
                                        line += " ";
                                        m += line.len();
                                        i += 1;
                                    }
                                    line.push(c);
                                    m += 1;
                                }

                                if line.len() > 0 {
                                    lines.push(line)
                                }

                                for line in lines {
                                    lock.1.push(line);
                                }
                            }
                            OpCode::User => {
                                let username = match String::from_utf8(decrypted) {
                                    Ok(s) => s,
                                    Err(_) => continue,
                                };
                                arcs.users.lock().unwrap().insert(username.clone());
                                presence_map
                                    .lock()
                                    .unwrap()
                                    .insert(username, Instant::now() + PRESENCE_TIMER);
                            }
                            OpCode::Leave => {
                                let username = match String::from_utf8(decrypted) {
                                    Ok(s) => s,
                                    Err(_) => continue,
                                };
                                arcs.users.lock().unwrap().remove(&username);
                            }
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
}

fn presence_manager(
    presences: Arc<Mutex<HashMap<String, Instant>>>,
    users: Arc<Mutex<HashSet<String>>>,
) {
    loop {
        let mut presences_lock = presences.lock().unwrap();
        let mut users_lock = users.lock().unwrap();
        let mut min_instant = Instant::now() + Duration::from_secs(10);
        let mut to_del = Vec::new();
        for (s, i) in &*presences_lock {
            min_instant = min(min_instant, *i);
            if *i < Instant::now() {
                to_del.push(s.clone());
            }
        }
        for s in to_del {
            presences_lock.remove(&s);
            users_lock.remove(&s);
        }
        drop(presences_lock);
        drop(users_lock);
        std::thread::sleep(min_instant - Instant::now());
    }
}

fn udp_sender(socket: Arc<UdpSocket>, rx: Receiver<Op>, room: String) {
    loop {
        let msg = rx.recv().unwrap();
        match msg {
            Op::User(opcode, username) => {
                let mut to_send = Vec::new();
                to_send.extend_from_slice(SIGNATURE.as_bytes());
                let mut to_encrypt = Vec::new();
                to_encrypt.push(opcode as u8);
                to_encrypt.extend_from_slice(username.as_bytes());
                let encrypted = if let Ok(v) = encrypt(&to_encrypt, room.as_bytes()) {
                    v
                } else {
                    continue;
                };
                to_send.extend_from_slice(&encrypted);
                socket.send(&to_send);
            }
            Op::Leave(opcode, username) => {
                let mut to_send = Vec::new();
                to_send.extend_from_slice(SIGNATURE.as_bytes());
                let mut to_encrypt = Vec::new();
                to_encrypt.push(opcode as u8);
                to_encrypt.extend_from_slice(username.as_bytes());
                let encrypted = if let Ok(v) = encrypt(&to_encrypt, room.as_bytes()) {
                    v
                } else {
                    continue;
                };
                to_send.extend_from_slice(&encrypted);
                socket.send(&to_send);
            }
            Op::Message(opcode, username, msg) => {
                let mut to_send = Vec::new();
                to_send.extend_from_slice(SIGNATURE.as_bytes());
                let mut to_encrypt = Vec::new();
                to_encrypt.push(opcode as u8);
                to_encrypt.extend_from_slice(username.as_bytes());
                to_encrypt.push(0);
                to_encrypt.extend_from_slice(msg.as_bytes());
                let encrypted = if let Ok(v) = encrypt(&to_encrypt, room.as_bytes()) {
                    v
                } else {
                    continue;
                };
                to_send.extend_from_slice(&encrypted);
                socket.send(&to_send);
            }
        }
    }
}
