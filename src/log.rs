use crate::net::NetOpts;
use std::{collections::HashMap, hash::Hash};
use std::cmp::Ordering;

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
    arg_lines.insert("Listener Port".to_string(), __netopts.clients.to_string());
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
        println!("value: {}", __value);
        s.push_str(
            &format!("\t- {0:<1$}: {2}\n", __key, max_len, __value)
        );
    }    
    println!("{s}");
}

/*
    motd format
    Starting OpenWings v0.1.0
    Args:
        - Broadcast Port: 127.0.0.1:25372
        - Listener Port: 127.0.0.1.25373
        - Settings Path:
        - Game ID:
*/
