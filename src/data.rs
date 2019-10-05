use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Guild {
    id: u64,
    debates: Vec<Debate>,
    slowmode: u64
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