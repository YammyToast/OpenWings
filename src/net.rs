use crossterm::{cursor, ExecutableCommand};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_stream::StreamExt;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::sync::Arc;
use std::{collections::HashMap, collections::VecDeque, hash::Hash, net::SocketAddr};
use tokio::net::{TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use futures::sink::SinkExt;
use chrono::{DateTime, Utc};
use uuid::{uuid, Uuid};


use crate::game::{Game, Player};


#[derive(Debug, Deserialize)]
pub struct JSONSettings {
    pub players: usize,
}
type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

// ===========================================================

pub struct Shared{
    pub game_id: String,
    clients: HashMap<SocketAddr, Tx>,
    pub message_buf: VecDeque<(SocketAddr, String)>,
    pub registered_users: HashMap<Uuid, Player>
}

pub struct Client {
    lines: Framed<TcpStream, LinesCodec>,
    rx: Rx
}
// ===========================================================


impl Shared{
    pub fn new(__game_id: &String) -> Self {
        Shared {
            game_id: __game_id.to_string(),
            clients: HashMap::new(),
            message_buf: VecDeque::new(),
            registered_users: HashMap::new(),
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

// ===========================================================

async fn process_client(__net_shared: Arc<Mutex<Shared>>, __stream: TcpStream, __addr: SocketAddr) -> Result<(), ()> {
    // let state = __net_shared.lock().await;
    let mut lines = Framed::new(__stream, LinesCodec::new());
    // let msg = format!("{{route:'srv-greetings', 'header': {{'game_id': {}, 'timestamp': {} }}}}", state.game_id, Utc::now().timestamp());
    let msg = "OpenWings!";
    lines.send(msg).await.unwrap();
    let mut client = Client::new(__net_shared.clone(), lines).await.unwrap();

    loop {
        tokio::select! {
            Some(msg) = client.rx.recv() => {
                client.lines.send(&msg).await.unwrap();
            }
            result = client.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = __net_shared.lock().await;
                    
                    // Do some checking of ports here!
                    state.message_buf.push_back((__addr, msg.clone()));

                    let msg = format!("{}: {}", __addr, msg);
                    state.broadcast(&msg).await;
                }
                Some(Err(e)) => {
                    println!("MESSAGE ERROR");
                }
                None => break,
            },
        }
    }
    // Clean Disconnection
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

// ===========================================================

pub async fn handle_connections(__game: &Game<'_>) {
    let net_shared = Arc::clone(&__game.net_shared);
    let listener = Arc::clone(&__game.listener);
    tokio::spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            let client_shared = Arc::clone(&net_shared);
            tokio::spawn( async move {
                if let Err(e) = process_client(client_shared, stream, addr).await {
                    println!("Spawn Error: {:?}", e);
        
                }
            });

        }

    });
}



