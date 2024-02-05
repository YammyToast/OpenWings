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

use crate::game::Game;


#[derive(Debug, Deserialize)]
pub struct JSONSettings {
    pub players: u8,
}
type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

pub struct Shared{
    clients: HashMap<SocketAddr, Tx>,
    message_buf: VecDeque<(SocketAddr, String)>
}

pub struct Client {
    lines: Framed<TcpStream, LinesCodec>,
    rx: Rx
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            clients: HashMap::new(),
            message_buf: VecDeque::new()
        }
    }

    async fn broadcast(&mut self, message: &str) {
        for client in self.clients.iter_mut() {
            let _ = client.1.send(message.into());
        }
    }
}

impl Client {
    async fn new(__net_shared: Arc<Mutex<Shared>>, __lines: Framed<TcpStream, LinesCodec>) -> io::Result<Client> {
        let addr = __lines.get_ref().peer_addr().unwrap();
        let (tx, rx) = mpsc::unbounded_channel();
        
        __net_shared.lock().await.clients.insert(addr, tx);
        
        Ok(Client { lines:__lines, rx: rx })
    }
}

async fn process_client(__net_shared: Arc<Mutex<Shared>>, __stream: TcpStream, __addr: SocketAddr) -> Result<(), ()> {
    let mut lines = Framed::new(__stream, LinesCodec::new());
    lines.send("OpenWings!").await.unwrap();
    println!("HERE");
    let mut client = Client::new(__net_shared.clone(), lines).await.unwrap();

    loop {
        tokio::select! {
            Some(msg) = client.rx.recv() => {
                client.lines.send(&msg).await.unwrap();
            }
            result = client.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = __net_shared.lock().await;

                }
                Some(Err(e)) => {
                    println!("ERROR");
                }
                None => break,
            },
        }
    }

    // If this section is reached it means that the client was disconnected!
    // Let's let everyone still connected know about it.
    {
        let mut net_shared = __net_shared.lock().await;
        net_shared.clients.remove(&__addr);
    }


    Ok(())
}


// }
pub struct NetOpts {
    pub listen: SocketAddrV4,
    pub game_settings_loc: Box<Path>,
    pub id: String,
}

impl NetOpts {
    pub fn new(
        mut __clients_str: Option<String>,
        mut __settings_path: Option<String>,
        mut __id: Option<String>,
    ) -> NetOpts {
        let clients_socket_raw: &mut String = __clients_str.get_or_insert("25373".to_string());
        

        let clients_socket: SocketAddrV4 = SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            clients_socket_raw.parse::<u16>().unwrap(),
        );
        let settings_loc = std::path::Path::new(
            __settings_path.get_or_insert("./settings_default.json".to_string()),
        );

        let id = __id.expect("No Game ID Provided! (-i ID)");

        return NetOpts {
            listen: clients_socket,
            game_settings_loc: settings_loc.into(),
            id: id,
        };
    }
}


pub async fn handle_connections(__game: &Game<'_>) {
    let (stream, addr) = __game.listener.accept().await.unwrap();
    let net_shared = Arc::clone(&__game.net_shared);
    tokio::spawn(async move {
        if let Err(e) = process_client(net_shared, stream, addr).await {
            println!("{:?}", e);

        }

    });
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
