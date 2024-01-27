use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::{collections::HashMap, hash::Hash, collections::VecDeque,};
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
    pub players: HashMap<String, SocketAddrV4>,
    pub ready: bool,
    net_opts: &'a NetOpts,
    msg_queue: VecDeque<String>
}

impl Lobby<'_> {
    pub fn new<'a>(__json_settings: &'a JSONSettings, __netops: &'a NetOpts) -> Lobby<'a> {
        let players= __json_settings.players.clamp(0, 5);
        return Lobby {
            player_count: players,
            players: HashMap::with_capacity(players.into()),
            ready: false,
            net_opts: __netops,
            msg_queue: VecDeque::new(),
        }
    }   

    pub fn open_connections_blocking(&mut self) {
        let mut stdout = std::io::stdout();

        let capacity: u16 = self.player_count.into();
        let players_conn: usize = self.players.len();
        while self.players.len() != self.players.capacity() {
            self.players.clear();
            for i in (1..5) {
                self.players.insert(i.to_string(), SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 88));
                display_blocking(&stdout, &self, &capacity, &i);
            }

        }
    }
}