use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{DateTime, Utc};
use crate::game::{Game, Player};
use std::{collections::HashMap};
// ===========================================================

// pub const CL_MESSAGE_INDEX: HashMap<String, fn(&Game, &str)->Result<Box<dyn ClMessage>, ()>> = HashMap::from([
//     ("cl-req-register".to_string(), ClRequestRegister::deserialize)

// ]);

// ===========================================================

trait JSONBody {
    fn get_route(&self) -> String;
    fn serialize(&self) -> String;
    // fn deserialize(&self) -> String;
}

// ===========================================================

struct SrvMessage {
    route: String,
    header: MessageHeader,
    body: Box<dyn JSONBody>
}

impl SrvMessage {
    pub fn new(__header: MessageHeader, __body: Box<dyn JSONBody>) -> Self {
        SrvMessage {
            route: __body.get_route(),
            header: __header,
            body: __body
        }
    }

    pub fn serialize(&self) -> String {
        let msg = json!({
            "route": self.route,
            "header": self.header.serialize(),
            "body": self.body.serialize()
        });
        msg.to_string()
    }
}

// ===========================================================

#[derive(Debug)]
pub struct MessageHeader {
    pub game_id: String,
    pub timestamp: i64 // UNIXepoch
}

impl MessageHeader {
    pub fn new(__game: &Game) -> Self {
        let dt_now = Utc::now();
        return MessageHeader {
            game_id: __game.netopts.id.clone(),
            timestamp: dt_now.timestamp() 
        }
    }

    pub fn serialize(&self) -> String {
        let msg = json!({
            "game_id": self.game_id,
            "timestamp": self.timestamp
        });
        msg.to_string()
    }

    pub fn deserialize(__json: serde_json::Value) -> Result<Self, ()> {
        Ok(MessageHeader{
            game_id: "1".to_string(),
            timestamp: 1

        })
    }

}

// ===========================================================

pub struct SrvGreetings {
    pub current_players: usize,
    pub game_settings: String
}

impl JSONBody for SrvGreetings {
    fn serialize(&self) -> String {
        let msg = json!({
            "current_players": self.current_players,
            "game_settings": self.game_settings
        });
        msg.to_string()
    }

    fn get_route(&self) -> String {
        return "srv-greetings".to_string()
    }
}

// ===========================================================

pub struct SrvSuccessfulRegister {
    pub req_uuid: String,
    pub new_uuid: String
}

impl JSONBody for SrvSuccessfulRegister {
    fn serialize(&self) -> String {
        let msg = json!({
            "req_uuid": self.req_uuid,
            "new_uuid": self.new_uuid
        });
        msg.to_string()
    }

    fn get_route(&self) -> String {
        return "srv-register-succeed".to_string()
    }

}

// ===========================================================

pub struct SrvFailRegister {
    pub req_uuid: String,
    pub err: String
}

impl JSONBody for SrvFailRegister {
    fn serialize(&self) -> String {
        let msg = json!({
            "req_uuid": self.req_uuid,
            "err": self.err
        });
        msg.to_string()  
    }

    fn get_route(&self) -> String {
        return "srv-register-fail".to_string()
    }

}

// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================
// ===========================================================

trait ClMessage {
    fn deserialize(__game: &Game, __json: &str) -> Result<Box<Self>, ()> where Self: Sized;
}

pub struct ClRequestRegister {
    username: String,
    req_uuid: String
}

impl ClMessage for ClRequestRegister {
    fn deserialize(__game: &Game, __json: &str) -> Result<Box<Self>, ()> {
        Err(())
    }

}

