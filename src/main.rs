use bevy::prelude::*;

pub mod world;
pub mod camera;
pub mod player;
pub mod postprocess;
pub mod ui;

use world::WorldPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use postprocess::PostProcessPlugin;
use ui::FpsCounter;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            PostProcessPlugin,
            FpsCounter,
        ))
        .run();
}
