use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;

pub struct NetOpts {
    broadcast: SocketAddrV4,
    clients: SocketAddrV4,
    game_settings_loc: Box<Path>,
    id: u16,
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
            .expect("No Game ID Provided! (-i ID)")
            .parse::<u16>()
            .expect("Invalid Game ID Provided");

        return NetOpts {
            broadcast: broadcast_socket,
            clients: clients_socket,
            game_settings_loc: settings_loc.into(),
            id: id,
        };
    }
}
