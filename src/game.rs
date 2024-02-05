use std::task::Poll;
use std::sync::Arc;

use crossterm::event::{self, KeyCode, KeyEventKind};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::net::{self, JSONSettings, NetOpts, Shared};
use crate::log::{lobby_display, term, UpdateError, UpdateErrorTypes};

pub enum GameStates {
    Lobby,
    Loop
}

pub enum PollEventResults {
    None,
    Break
}

pub struct Game<'a> {
    pub state: GameStates,
    pub netopts: &'a NetOpts,
    pub players: u8,
    pub listener: TcpListener,
    pub net_shared: Arc<Mutex<Shared>>,
}

impl Game<'_> {
    pub async fn new<'a>(__netopts: &'a NetOpts, __settings: &'a JSONSettings) -> Game<'a> {
        let listener: TcpListener = match TcpListener::bind(__netopts.listen).await {
            Ok(e) => e,
            Err(_) => panic!("Can't Bind Listening Port: {}", __netopts.listen)
        };

        let net_shared = Arc::new(Mutex::new(Shared::new()));

        Game {
            state: GameStates::Lobby,
            netopts: __netopts,
            players: __settings.players,
            listener: listener,
            net_shared: net_shared
        }

    }

    pub fn update_display(&mut self, __term: &mut term) {
        match self.state {
            GameStates::Lobby => {
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
            GameStates::Lobby => {
                net::handle_connections(self).await;
                None

            }
            _ => {
                Some(UpdateError { err_type: UpdateErrorTypes::UnknownState })

            }
        };
    }




}