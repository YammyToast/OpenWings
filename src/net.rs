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
    pub players: u8,
}
type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

pub struct Shared{
    clients: HashMap<SocketAddr, Tx>,
    message_buf: VecDeque<(SocketAddr, String)>,
    registered_users: HashMap<Uuid, Player>
}

pub struct Client {
    lines: Framed<TcpStream, LinesCodec>,
    rx: Rx
}

impl Shared{
    pub fn new() -> Self {
        Shared {
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

async fn process_client(__net_shared: Arc<Mutex<Shared>>, __stream: TcpStream, __addr: SocketAddr) -> Result<(), ()> {
    let mut lines = Framed::new(__stream, LinesCodec::new());
    
    // let greetings = JSONMessage::new(
    //     ,
    //     body
    // )
    lines.send("OpenWings!").await.unwrap();
    let mut client = Client::new(__net_shared.clone(), lines).await.unwrap();

    loop {
        tokio::select! {
            Some(msg) = client.rx.recv() => {
                client.lines.send(&msg).await.unwrap();
            }
            result = client.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = __net_shared.lock().await;
                    let msg = format!("{}: {}", __addr, msg);
                    state.broadcast(&msg).await;
                }
                Some(Err(e)) => {
                    println!("ERROR");
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

// ===========================================================

struct JSONMessage {
    header: MessageHeader,
    body: Box<dyn JSONBody>
}

impl JSONMessage {
    pub fn new(__header: MessageHeader, __body: Box<dyn JSONBody>) -> Self {
        JSONMessage {
            header: __header,
            body: __body
        }
    }
}

// ===========================================================

trait JSONBody {
    fn serialize(&self) -> String;
    // fn deserialize(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeader {
    pub game_id: String,
    pub port: String,
    pub timestamp: i64 // UNIXepoch
}

impl MessageHeader {
    pub fn new(__game: &Game) -> Self {
        let dt_now = Utc::now();
        return MessageHeader {
            game_id: __game.netopts.id.clone(),
            port: __game.netopts.listen.to_string(),
            timestamp: dt_now.timestamp() 
        }
    }
}

// ===========================================================

pub struct BodyGreetings {
    pub current_players: u8,
    pub game_settings: String
}

impl BodyGreetings {
    pub fn new(__game: &Game) -> Self {
        BodyGreetings {
            current_players: 0,
            game_settings: "temp".into()
        }
    }
}

impl JSONBody for BodyGreetings {
    fn serialize(&self) -> String {
        let txt = json!({
            "current_players" : self.current_players,
            "game_settings": self.game_settings
        });
        return serde_json::to_string(&txt).unwrap();
    }
}

// ===========================================================

