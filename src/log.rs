use crate::net::NetOpts;


/* Log Line Format
     Timestamp | Line Number | [Message]
*/
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn display_motd(__netopts: &NetOpts) {
    let mut s = String::new();
    s.push_str(&format!("Starting OpenWings v{0}", VERSION).to_string());


    println!("{s}");
}


/*
    motd format

*/