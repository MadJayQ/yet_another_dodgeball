use bevy::prelude::*;

extern crate dodgeball_core;
use crate::dodgeball_core::{dodgeball, logging};


fn main() {
    let mut app = App::new();
    // println!("Enabling logging subsystem");
    // logging::init_logging(&mut app);
    
    app.add_plugins(DefaultPlugins)
        .add_plugins(dodgeball::DodgeballPlugin);
    
    bevy::log::info!("Starting dodgeball runtime...");
    app.run();
}
