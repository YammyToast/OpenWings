use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

extern crate getopts;
use getopts::Options;
use net::Lobby;

mod log;
use crate::net::{NetOpts, JSONSettings};
mod net;

use std::{fs::File, io::Read};

use serde::Deserialize;
/***
 * Program Opts:
 * ------
 * Broadcast Socket - Output -o {16 bit integer, i.e. 88, 8000, 3000}
 * Listener Socket - Input -l {16 bit integer}
 * Game Settings JSON/YAML location - FileLoc -set
 * Game ID - ID -i {16 bit integer}
 */



fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program: String = args[0].clone();

    // Options Parser init
    let mut opts: Options = Options::new();

    // Define Options and Help Info
    opts.optopt(
        "o",
        "output-socket",
        "set output socket address",
        "OUTPUT_SOCKET",
    );
    opts.optopt(
        "l",
        "listen-socket-range",
        "set list of listening sockets",
        "INPUT_SOCKET,INPUT_SOCKET,...",
    );
    opts.optopt(
        "s",
        "settings-init-loc",
        "file location of initial game settings",
        "FILE_LOCATION",
    );
    opts.optopt(
        "i",
        "game-id",
        "unique identifier for this game process",
        "ID",
    );
    opts.optflag("h", "help", "print this help menu");

    // Parse Matches from full arguments list
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let broad_binding: Option<String> = matches.opt_str("o");
    let listen_binding: Option<String> = matches.opt_str("l");
    // let listen_socket_raw: &mut String = listen_binding.get_or_insert("25373".to_string());
    let settings_loc_binding: Option<String> = matches.opt_str("s");
    let id_binding: Option<String> = matches.opt_str("i");

    let netopts = NetOpts::new(
        broad_binding,
        listen_binding,
        settings_loc_binding,
        id_binding,
    );

    // Load Settings File Data
    let mut settings_file  = File::open(netopts.game_settings_loc.clone())
    .expect(&format!(
        "Could not open file {}",
        (*netopts.game_settings_loc)
            .to_str()
            .unwrap()
            .to_string(),
    ));
    let mut cts = String::new();
    settings_file.read_to_string(&mut cts).unwrap();
    let json: JSONSettings = serde_json::from_str(&cts.to_string()).expect("Malformed JSON in provided Settings file.");

    // Display Nice Looking Message :)
    // This looks cool no other reason.
    log::display_motd(&netopts);

    let mut lobby: Lobby = Lobby::new(&json, &netopts);
    lobby.open_player_registration().await;

}

// use std::{
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
// };
// fn main() {
//     // Binding may throw error on initialization and this should be handled
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream)
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//     .lines()
//     .map(|result| result.unwrap())
//     .take_while(|line| !line.is_empty())
//     .collect();
//     println!("Request: {:#?}", http_request);

//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write_all(response.as_bytes()).unwrap();

// }
