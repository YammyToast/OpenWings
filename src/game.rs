use std::task::Poll;

use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::net::{JSONSettings, NetOpts};
use crate::log::{lobby_display, term};

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
    pub players: u8
}

impl Game<'_> {
    pub fn new<'a>(__netopts: &'a NetOpts, __settings: &'a JSONSettings) -> Game<'a> {
        Game {
            state: GameStates::Lobby,
            netopts: __netopts,
            players: __settings.players
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

    pub fn update() -> Result<_, Box<dyn > {


    }

}