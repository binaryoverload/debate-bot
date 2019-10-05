use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Guild {
    id: u64,
    debates: Vec<Debate>
}


#[derive(Serialize, Deserialize, Debug)]
struct Debate {
    name: &str,
    slug: &str,
    channel_id: u64
}