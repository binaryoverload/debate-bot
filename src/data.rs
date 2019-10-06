use serde::{Serialize, Deserialize};

use wither::Model;

use std::collections::HashMap;

#[derive(Model, Serialize, Deserialize, Debug)]
struct Guild {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<mongodb::oid::ObjectId>,
    #[model(index(index="dsc", unique="true"))]
    pub guild_id: u64,
    pub debates: Vec<Debate>,
    pub slowmode: u64
}

impl Guild {
    fn new(id: u64) {
        Guild {
            id,
            vec!(),
            0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Debate {
    name: &str,
    slug: &str,
    channel_id: u64,
    close_time: u64,
    mode: DebateMode,
    users: HashMap<u64, User>
}

impl Debate {
    fn new(name: &str, channel_id: u64, mode: DebateMode) {
        Debate {
            name,
            name.to_lowercase().replace(" ", "-"),
            channel_id,
            0,
            DebateMode::FFA,
            HashMap::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    warnings: u8
}

enum DebateMode {
    VS, FFA
}