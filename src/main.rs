use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{pbr::wireframe::WireframePlugin, prelude::*, window::WindowResolution};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod camera;
pub mod generation;
pub mod mouse_grab;
pub mod postprocess;
pub mod ui;
pub mod world;

use camera::CameraPlugin;
use generation::GenerationPlugin;
use mouse_grab::MouseGrabPlugin;
use ui::FpsCounter;
use world::WorldPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Terrain Generation".to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            AtmospherePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            WireframePlugin,
            MouseGrabPlugin,
            CameraPlugin,
            GenerationPlugin,
            WorldPlugin,
            NoCameraPlayerPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            FpsCounter,
        ))
        .insert_resource(MovementSettings {
            sensitivity: 0.00010,
            speed: 100.0,
        })
        .insert_resource(KeyBindings {
            move_forward: KeyCode::KeyW,
            move_left: KeyCode::KeyA,
            move_backward: KeyCode::KeyS,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            ..Default::default()
        })
        .run();
}
