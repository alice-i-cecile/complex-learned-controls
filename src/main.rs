use bevy::prelude::*;

mod building_blocks;
mod controls;
mod initialization;
mod physics;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::initialization::InitializationPlugin)
        .run();
}
