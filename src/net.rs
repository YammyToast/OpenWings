use crossterm::{cursor, ExecutableCommand};
use serde::Deserialize;
use tokio_stream::StreamExt;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::sync::Arc;
use std::{collections::HashMap, collections::VecDeque, hash::Hash, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use futures::sink::SinkExt;


#[derive(Debug, Deserialize)]
pub struct JSONSettings {
    pub players: u8,
}


// type Tx = mpsc::UnboundedSender<String>;
// type Rx = mpsc::UnboundedReceiver<String>;

// struct Shared {
//     peers: HashMap<SocketAddr, Tx>,
// }

// struct Peer {
//     lines: Framed<TcpStream, LinesCodec>,
//     rx: Rx,
// }

// impl Shared {
//     fn new() -> Self {
//         Shared {
//             peers: HashMap::new(),
//         }
//     }

//     async fn broadcast(&mut self, message: &str) {
//         for peer in self.peers.iter_mut() {
//             let _ = peer
//                 .1
//                 .send(message.into())
//                 .expect("Cannot Broadcast Message.");
//         }
//     }

//     async fn selective_broadcast(&mut self, __addr: SocketAddr, message: &str) {
//         for peer in self.peers.iter_mut() {
//             if *peer.0 != __addr {
//                 let _ = peer
//                 .1
//                 .send(message.into())
//                 .expect("Cannot Broadcast Message.");
//             }

//         }
//     }
// }

// impl Peer {
//     async fn new (__state: Arc<Mutex<Shared>>, __lines: Framed<TcpStream, LinesCodec>) -> io::Result<Peer> {
//         let addr = __lines.get_ref().peer_addr().unwrap();

//         let (tx, rx) = mpsc::unbounded_channel();

//         __state.lock().await.peers.insert(addr,tx);
//         Ok(Peer {lines: __lines , rx})
//     }

// }
pub struct NetOpts {
    pub broadcast: SocketAddrV4,
    pub listen: SocketAddrV4,
    pub game_settings_loc: Box<Path>,
    pub id: String,
}

impl NetOpts {
    pub fn new(
        mut __broadcast_str: Option<String>,
        mut __clients_str: Option<String>,
        mut __settings_path: Option<String>,
        mut __id: Option<String>,
    ) -> NetOpts {
        let broadcast_socket_raw: &mut String = __broadcast_str.get_or_insert("25372".to_string());
        let clients_socket_raw: &mut String = __clients_str.get_or_insert("25373".to_string());
        let broadcast_socket: SocketAddrV4 = SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            broadcast_socket_raw.parse::<u16>().unwrap(),
        );
        let clients_socket: SocketAddrV4 = SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            clients_socket_raw.parse::<u16>().unwrap(),
        );

        let settings_loc = std::path::Path::new(
            __settings_path.get_or_insert("./settings_default.json".to_string()),
        );

        let id = __id.expect("No Game ID Provided! (-i ID)");

        return NetOpts {
            broadcast: broadcast_socket,
            listen: clients_socket,
            game_settings_loc: settings_loc.into(),
            id: id,
        };
    }
}

// pub struct Lobby<'a> {
//     pub player_count: u8,
//     pub players: HashMap<String, SocketAddr>,
//     pub ready: bool,
//     net_opts: &'a NetOpts,
// }


/***
 * Player Register Packet
 * Player ID | IP (Is this inferred??) | Client ID (Optional)
 */
