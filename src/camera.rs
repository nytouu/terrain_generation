use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_flycam::FlyCam;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub sensitivity: f32,
    pub scroll_sensitivity: f32,
    pub rotate_lock: f32,
    pub rotation: Vec3,

    pub focus: Vec3,
    pub distance: f32,
    pub min_distance: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        CameraController {
            sensitivity: 0.5,
            scroll_sensitivity: 20.0,
            rotate_lock: 1.80,
            rotation: Vec3::new(0.0, 0.0, 0.0), 
            focus: Vec3::ZERO,
            distance: 100.0, 
            min_distance: 10.0
        }
    }
}

fn setup_camera(mut commands: Commands){
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 50.0_f32.to_radians(),
                ..default()
            }.into(),
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            rotate_lock: 88.0 * 0.0174533,
            sensitivity: (0.173) / 500.0,
            ..default()
        },
        AtmosphereCamera::default(),
        FogSettings {
            color: Color::rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 300.0,
            falloff: FogFalloff::Linear {
                start: 128.0,
                end: 1024.0,
            },
        },
        FlyCam
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 10.0, 20.0)),
        ..default()
    });
}
