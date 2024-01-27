use core::time;
use std::io::{Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::thread;
use std::{collections::HashMap, hash::Hash, collections::VecDeque, net::SocketAddr};
use crossterm::{
    cursor,
    ExecutableCommand,
};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, BufReader};
use tokio::net::{TcpListener, TcpStream};


use serde::Deserialize;

use crate::log::display_blocking;

#[derive(Debug, Deserialize)]
pub struct JSONSettings {
    players: u8
}

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

        let id = __id
            .expect("No Game ID Provided! (-i ID)");

        return NetOpts {
            broadcast: broadcast_socket,
            listen: clients_socket,
            game_settings_loc: settings_loc.into(),
            id: id,
        };
    }
}

pub struct Lobby<'a> {
    pub player_count: u8,
    pub players: HashMap<String, SocketAddr>,
    pub ready: bool,
    net_opts: &'a NetOpts,
}

impl Lobby<'_> {
    pub fn new<'a>(__json_settings: &'a JSONSettings, __netops: &'a NetOpts) -> Lobby<'a> {
        let players= __json_settings.players.clamp(0, 5);
        return Lobby {
            player_count: players,
            players: HashMap::with_capacity(players.into()),
            ready: false,
            net_opts: __netops,
        }
    }   

    async fn handle_conn(mut __socket: TcpStream, __id: &String) {
        let (read_stream, mut write_stream) = __socket.split();
        let mut read_stream = BufReader::new(read_stream);
        loop {
            let mut data = String::new();
            let read = read_stream.read_line(&mut data).await.unwrap();
            if read == 0 {
                break;
            }

            println!("data: {}", data);

            let res_bytes = format!("OpenWings: {__id}", ).as_bytes();
        }
    }

    pub async fn open_player_registration(&mut self) {
        let mut stdout = std::io::stdout();
        let mut players_conn: u8 = 0;
        let capacity: u8 = self.player_count.into();

        let listener = match TcpListener::bind(self.net_opts.listen).await {
            Ok(e) => e,
            Err(_) => panic!("Can't Bind Listening Port: {}", self.net_opts.listen) 
        };

        while players_conn != capacity {
            let (socket, ip) = listener.accept().await.unwrap();

            self.players.insert(players_conn.to_string(), ip);
            players_conn = players_conn + 1;
            let id = &self.net_opts.id;

            tokio::spawn(async move {
                Self::handle_conn(socket, id).await
            });
            display_blocking(&stdout, &self, &capacity, &players_conn)
            // for i in 1..6 {
            //     players_conn = players_conn + 1;
            //     display_blocking(&stdout, &self, &capacity, &i);
            //     thread::sleep(time::Duration::from_millis(1000));
            // }
        }
        // Very Nasty Code but it works :)
        // Cleanup Terminal Outputs.
        stdout.execute(cursor::MoveToNextLine((capacity).into())).unwrap();
        stdout.write_all("\n".as_bytes()).unwrap();
    }
}


/***
 * Player Register Packet
 * Player ID | IP (Is this inferred??) | Client ID (Optional)
 */