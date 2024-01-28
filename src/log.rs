use crate::{game::Game, net::NetOpts};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::*,
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::{block::Title, Block, Borders, Paragraph},
};
use std::cmp::Ordering;
use std::io::{stdout, Result, Stdout};
use std::{collections::HashMap, hash::Hash, thread, time};

pub type term = Terminal<CrosstermBackend<Stdout>>;

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
        s.push_str(&format!("\t- {0:<1$}: {2}\n", __key, max_len, __value));
    }

    println!("{s}");
}

pub fn init_terminal() -> Result<term> {
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();
    return Ok(terminal);
}

pub fn term_setup() {
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}

pub fn term_clear() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn lobby_display(__term: &mut term, __game: &Game) {
    __term
        .draw(|frame| {
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(7),
                    Constraint::Min(9),
                    Constraint::Min(15),
                ])
                .split(frame.size());

            let header = build_header(__game.netopts, &__game.players);
            let list = build_player_list(&__game.players);
            let settings = build_game_settings();
            frame.render_widget(
                header.block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(Title::from(format!("[OpenWings v{0}]", VERSION))),
                ),
                layout[0],
            );
            frame.render_widget(
                list.block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(Title::from("[Lobby]")),
                ),
                layout[1],
            );
            frame.render_widget(
                settings.block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(Title::from(format!("[Game Settings]"))),
                ),
                layout[2],
            )
        })
        .unwrap();
}

fn build_header<'a>(__netotps: &'a NetOpts, __players: &'a u8) -> Paragraph<'a> {
    let mut s: String = String::new();
    s.push_str(&*format!("\t- Broadcast Port: {:?}\n", __netotps.broadcast));
    s.push_str(&*format!(
        "\t- Listener/Clients Port: {:?}\n",
        __netotps.listen
    ));
    s.push_str(&*format!("\t- Players: {:?}\n", __players.to_string()));
    return Paragraph::new(s);
}

fn build_player_list<'a>(__players: &'a u8) -> Paragraph<'a> {
    let mut s: String = String::new();
    s.push_str(&*format!("Waiting for Players: (0/{})", __players));
    return Paragraph::new(s);
}

fn build_game_settings<'a>() -> Paragraph<'a> {
    let mut s: String = String::new();
    s.push_str(&*format!("TODO"));
    return Paragraph::new(s);
}

// pub fn display_blocking(mut __stdout: &Stdout, __lobby: &Lobby, __capacity: &u8, __player_count: &u8) {
//     __stdout.queue(cursor::MoveToPreviousLine((*__capacity).into())).unwrap();

//     __stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

//     let mut vec_players: Vec<(String, SocketAddr)> = __lobby.players.clone().into_iter().collect();
//     for i in 1..*__capacity+1 {
//         match vec_players.pop() {
//             Some((id, socket)) => {
//                 __stdout.write_all(format!("Player: {:?} - Socket: {:?}\n", id, socket).as_bytes()).unwrap();
//             }
//             None => {
//                 __stdout.write_all("\n".as_bytes()).unwrap();
//             }
//         }

//     }
//     __stdout.write_all(format!("Waiting for Connections ({}/{})...", __player_count, __capacity).as_bytes()).unwrap();

//     __stdout.flush().unwrap();
//     __stdout.execute(cursor::Show).unwrap();

// }

/***
 * Display Blocking Fmt:
 * Player {ID} - {IP}
 * Waiting for Connections (1/5)...
 */
