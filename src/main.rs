use bevy::{prelude::*, window::WindowResolution, pbr::wireframe::WireframePlugin};
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod world;
pub mod camera;
pub mod player;
pub mod postprocess;
// pub mod ui;
pub mod mouse_grab;
pub mod mesh;

use world::WorldPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use postprocess::PostProcessPlugin;
// use ui::FpsCounter;
use mouse_grab::MouseGrabPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy game".to_string(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            // WireframePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            MouseGrabPlugin,
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            PostProcessPlugin,
            WorldInspectorPlugin::new(),
            // FpsCounter,
        ))
        .run();
}
