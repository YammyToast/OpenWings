use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

extern crate getopts;
use getopts::Options;
use serde::Deserialize;
use std::{fs::File, io::Read};
use std::{thread, time::Duration};

mod game;
mod log;
mod net;
mod messages;
use crate::net::{JSONSettings, NetOpts};
use game::Game;
use log::{init_terminal, term_clear, term_setup};

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

    let listen_binding: Option<String> = matches.opt_str("l");
    // let listen_socket_raw: &mut String = listen_binding.get_or_insert("25373".to_string());
    let settings_loc_binding: Option<String> = matches.opt_str("s");
    let id_binding: Option<String> = matches.opt_str("i");

    let netopts = NetOpts::new(
        listen_binding,
        settings_loc_binding,
        id_binding,
    );

    // Load Settings File Data
    let mut settings_file = File::open(netopts.game_settings_loc.clone()).expect(&format!(
        "Could not open file {}",
        (*netopts.game_settings_loc).to_str().unwrap().to_string(),
    ));
    let mut cts = String::new();
    settings_file.read_to_string(&mut cts).unwrap();
    let json: JSONSettings =
        serde_json::from_str(&cts.to_string()).expect("Malformed JSON in provided Settings file.");

        
    let mut game: Game = Game::new(netopts, &json).await;
    // Display Nice Looking Message :)
    // This looks cool no other reason.
    log::display_motd(game.netopts.clone());
    thread::sleep(Duration::from_millis(1000));



    let mut term = init_terminal().expect("Could not initialize terminal for display!");
    term_setup();
    // Main Program Cycle
    loop {
        game.update_vars().await;
        game.update_display(&mut term);
        match game.handle_display_events() {
            game::PollEventResults::Break => break,
            game::PollEventResults::None => {}
        };
        game.update().await;
        
    }
    // Clear Environment
    term_clear();
}
