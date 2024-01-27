use crate::net::{Lobby, NetOpts};
use std::io::{stdout, Stdout, Write};
use std::net::SocketAddrV4;
use std::ops::Index;
use std::vec;
use std::{collections::HashMap, hash::Hash, thread, time};
use std::cmp::Ordering;
use crossterm::{
    cursor,
    terminal,
    ExecutableCommand,
    QueueableCommand
};


/* Log Line Format
     Timestamp | Line Number | [Message]
*/
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn display_motd(__netopts: &NetOpts) {
    let mut s = String::new();
    s.push_str(&format!("Starting OpenWings v{0}\n", VERSION).to_string());
    s.push_str(&"Args:\n".to_string());

    // Append Arg Lines
    let mut arg_lines: HashMap<String, String> = HashMap::new();
    arg_lines.insert(
        "Broadcast Port".to_string(),
        __netopts.broadcast.to_string(),
    );
    arg_lines.insert("Listener Port".to_string(), __netopts.listen.to_string());
    arg_lines.insert(
        "Settings Path".to_string(),
        (*__netopts.game_settings_loc)
            .to_str()
            .expect("Invalid UTF-8 in path-name.")
            .to_string(),
    );
    arg_lines.insert("Game ID".to_string(), __netopts.id.to_string());
    // Get and Sort Keys for Equal Padding
    let mut keys: Vec<String> = arg_lines.clone().into_keys().collect();
    keys.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else if a.len() == b.len() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    // Get Max Length from front of sorted list.
    let max_len = keys.first().unwrap().len();
    
    for (__key, __value) in arg_lines.clone() {
        s.push_str(
            &format!("\t- {0:<1$}: {2}\n", __key, max_len, __value)
        );
    }    

    println!("{s}");
}

pub fn display_blocking(mut __stdout: &Stdout, __lobby: &Lobby, __capacity: &u8, __player_count: &u8) {

    __stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

    let mut vec_players: Vec<(String, SocketAddrV4)> = __lobby.players.clone().into_iter().collect();
    for i in 1..*__capacity+1 {
        match vec_players.pop() {
            Some((id, socket)) => {
                __stdout.write_all(format!("Player: {:?} - Socket: {:?}\n", id, socket).as_bytes()).unwrap();
            }
            None => {
                __stdout.write_all("\n".as_bytes()).unwrap();
            }
        }

    }
    __stdout.write_all(format!("Waiting for Connections ({}/{})...", __player_count, __capacity).as_bytes()).unwrap();
    __stdout.queue(cursor::MoveToPreviousLine((*__capacity).into())).unwrap();
    
    __stdout.flush().unwrap();    
    __stdout.execute(cursor::Show).unwrap();




}

/***
 * Display Blocking Fmt:
 * Player {ID} - {IP}
 * Waiting for Connections (1/5)...
 */