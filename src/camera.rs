use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_atmosphere::prelude::*;
use bevy_flycam::FlyCam;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(Msaa::Sample4)
            .insert_resource(AtmosphereModel::default());
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
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
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
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 1.0),
            directional_light_exponent: 300.0,
            // falloff: FogFalloff::Linear {
            //     start: 128.0,
            //     end: 1024.0,
            falloff: FogFalloff::from_visibility_colors(
                512.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
        FlyCam
    ));

    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.8,
        maximum_distance: 1000.0,
        ..default()
    }
    .build();

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config,
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 10.0, 20.0)),
        ..default()
    });
}
