use bevy::{prelude::*, window::WindowResolution, pbr::wireframe::WireframePlugin};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_procedural_grass::ProceduralGrassPlugin;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_flycam::prelude::*;

pub mod world;
pub mod camera;
// pub mod player;
pub mod postprocess;
pub mod ui;
pub mod mouse_grab;

use world::WorldPlugin;
use camera::CameraPlugin;
// use player::PlayerPlugin;
use postprocess::PostProcessPlugin;
use ui::FpsCounter;
use mouse_grab::MouseGrabPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy game".to_string(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            AtmospherePlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            // ProceduralGrassPlugin::default(),
            WireframePlugin,
            MouseGrabPlugin,
            // PlayerPlugin, // from flycam
            WorldPlugin,
            NoCameraPlayerPlugin,
            CameraPlugin,
            PostProcessPlugin,
            WorldInspectorPlugin::new(),
            FrameTimeDiagnosticsPlugin::default(),
            FpsCounter,
        ))
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 50.0, // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_forward: KeyCode::Z,
            move_left: KeyCode::Q,
            move_backward: KeyCode::S,
            move_right: KeyCode::D,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            ..Default::default()
        })
        .run();
}
