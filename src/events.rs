use serenity::prelude::EventHandler;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use log::{info, error};

use crate::SettingsContainer;

pub struct Handler;

impl EventHandler for Handler {

    fn ready(&self, ctx: Context, ready: Ready) {
        let data = ctx.data.read();
        let settings = match data.get::<SettingsContainer>() {
            Some(v) => v,
            None => {
                error!("Couldn't open settings!");
                return;
            },
        };
        info!("Connected as {} with prefix: {:?}", ready.user.tag(), settings.read().unwrap().command.prefix);
    }

}
