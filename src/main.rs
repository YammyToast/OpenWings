use std::env;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

extern crate getopts;
use getopts::Options;

mod log;

/***
 * Program Opts:
 * ------
 * Broadcast Socket - Output -o {16 bit integer, i.e. 88, 8000, 3000}
 * Reciever Socket Range - Input(s) -i {16 bit integer},{16}... (comma delim) 
 * Game Settings JSON/YAML location.
 */

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));

}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Arguments: {args:?}");
    let program: String = args[0].clone();

    let mut opts: Options = Options::new();

    opts.optopt("o", "output-socket", "set output socket address", "OUTPUT_SOCKET");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!("{}", e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }


    log::display_motd()
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
