#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod settings;
mod events;
mod commands;

use std::sync::Arc;

use serenity::{
    client::{
        Client,
        bridge::gateway::ShardManager
    },
    framework::{
        StandardFramework,
        standard::macros::group
    },
    prelude::*
};

use settings::Settings;
use events::Handler;
use commands::{
    ping::*,
    shutdown::*
};

use log::{info, error, debug};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct SettingsContainer;

impl TypeMapKey for SettingsContainer {
    type Value = Arc<Settings>;
}

lazy_static! {
    static ref SETTINGS : Arc<Settings> = Arc::from(Settings::new().unwrap());
}

group!({
    name: "general",
    options: {},
    commands: [ping],
});

group!({
    name: "admin",
    options: {},
    commands: [shutdown],
});

fn main() {

    simple_logger::init_with_level(log::Level::Info).unwrap();

    let settings = &*SETTINGS;
    let mut client = Client::new(&settings.discord.token, Handler).expect("Error creating the client!");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<SettingsContainer>(Arc::clone(&SETTINGS));
    }

    client.with_framework(StandardFramework::new()
    .configure(|c| c
        .prefix(&settings.command.prefix)
    )
    .before(|_, msg, command_name| {
        info!("Received command '{}' from user '{}'", command_name, msg.author.tag());
        true
    })
    .after(|_, msg, command_name, error| {
        match error {
            Ok(()) => debug!("Successfully processed command '{}' from user '{}'", command_name, msg.author.tag()),
            Err(why) => error!("Command '{}' returned error {:?}", command_name, why)
        }
    })
    .group(&GENERAL_GROUP)
    .group(&ADMIN_GROUP));

    if let Err(why) = client.start() {
        panic!("An error occurred while running the client: {:?}", why);
    }
}
