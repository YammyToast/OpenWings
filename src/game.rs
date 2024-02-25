use std::task::Poll;
use std::sync::Arc;

use crossterm::event::{self, KeyCode, KeyEventKind};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::net::{self, JSONSettings, NetOpts, Shared};
use crate::log::{lobby_display, term, UpdateError, UpdateErrorTypes};
use crate::messages::{MessageHeader};

pub enum GameStates {
    OpenLobby,
    WaitLobby,
    Loop,
    End
}

pub enum PollEventResults {
    None,
    Break
}

pub struct Game<'a> {
    pub state: GameStates,
    pub netopts: &'a NetOpts,
    pub player_cap: usize,
    pub player_count: usize,
    pub listener: Arc<TcpListener>,
    pub net_shared: Arc<Mutex<Shared>>,
}

impl Game<'_> {
    pub async fn new<'a>(__netopts: &'a NetOpts, __settings: &'a JSONSettings) -> Game<'a> {
        let listener: TcpListener = match TcpListener::bind(__netopts.listen).await {
            Ok(e) => e,
            Err(_) => panic!("Can't Bind Listening Port: {}", __netopts.listen)
        };

        let net_shared = Arc::new(Mutex::new(Shared::new(&__netopts.id)));

        Game {
            state: GameStates::OpenLobby,
            netopts: __netopts,
            player_cap: __settings.players,
            player_count: 0,
            listener: Arc::new(listener),
            net_shared: net_shared
        }

    }

    pub async fn update_vars(&mut self) {
        let mut state = self.net_shared.lock().await;
        self.player_count = state.registered_users.keys().len();
    }


    pub fn update_display(&mut self, __term: &mut term) {
        match self.state {
            GameStates::WaitLobby => {
                lobby_display(__term, &self)
            },
            GameStates::Loop => {

            },
            _ => {}
        }
    }

    pub fn handle_display_events(&mut self) -> PollEventResults {
        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return PollEventResults::Break
                }

            }

        }
        PollEventResults::None
    }

    pub async fn update(&mut self) -> Option<UpdateError> { 
        return match self.state {
            GameStates::OpenLobby => {
                net::handle_connections(self).await;
                self.state = GameStates::WaitLobby;
                None

            }
            GameStates::WaitLobby => {
                // let messages = state.message_buf;
                // Drain Messages and Process all!
                // To keep state change in the same space, if update returns True,
                // update lobby to begin game. Otherwise, keep waiting.
                self.update_wait_lobby().await;
                None
            }
            _ => {
                Some(UpdateError { err_type: UpdateErrorTypes::UnknownState })

            }
        };
    }

    pub async fn update_wait_lobby(&mut self) -> bool {
        let mut state = self.net_shared.lock().await;
        for (addr, msg) in state.message_buf.drain(..) {
            println!("MESSAGE FROM {}: {}",addr, msg);
        }
        // Return false as the lobby is not ready yet!
        return false
    }

    // fn broadcast_message(&mut self, ) {


    // }

    pub fn create_message_header(&mut self) -> MessageHeader {
        return MessageHeader::new(self)
    }


}

// ===============================================================================

pub struct Player {


}