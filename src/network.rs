use std::{
    cmp::min,
    collections::{BTreeMap, HashMap, HashSet},
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    sync::{mpsc::Receiver, Arc, Mutex},
    time::{Duration, Instant},
};

use simple_crypt::decrypt;

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
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, PORT))?;
    socket.set_broadcast(true)?;
    socket.connect(SocketAddr::from((Ipv4Addr::BROADCAST, PORT)))?;

    let presence_map = Arc::new(Mutex::new(HashMap::<String, Instant>::new()));

    let mut read_buf: Vec<u8> = [0; 65536].to_vec();

    let presence_map_clone = presence_map.clone();
    let users_clone = arcs.users.clone();

    std::thread::spawn(|| presence_manager(presence_map_clone, users_clone));

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
                                    .insert(Instant::now() + PRESENCE_TIMER, username);
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
        let presences_lock = presences.lock().unwrap();
        let mut min_instant = ;
        for (s, i) in &*presences_lock {
            min_instant = min(min_instant, i);
        }
    }
}
