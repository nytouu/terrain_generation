use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}, pbr::CascadeShadowConfigBuilder};

use crate::{player::Player, postprocess::PostProcessSettings};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (
                sync_orbit,
                camera_orbit,
                camera_zoom
        ));
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
            scroll_sensitivity: 5.0,
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
                ..Default::default()
            }.into(),
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PostProcessSettings {
            intensity: 0.03,
            ..default()
        },
        CameraController {
            rotate_lock: 88.0 * 0.0174533,
            sensitivity: (0.173) / 500.0,
            ..Default::default()
        }
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 10.0, 20.0)),
        ..Default::default()
    });
}

fn camera_zoom(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>
){
    for (mut transform, mut controller) in camera_query.iter_mut(){
        for ev in ev_scroll.read() {
            controller.distance -= ev.y * controller.scroll_sensitivity;

            if controller.distance <= controller.min_distance {
                controller.distance = controller.min_distance;
            }

            transform.rotation.x =
                f32::clamp(transform.rotation.x, -controller.rotate_lock, controller.rotate_lock);

            let matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                controller.focus + matrix.mul_vec3(Vec3::new(0.0, 0.0, controller.distance));
        }
    }
}

fn camera_orbit(
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
){
    for (mut transform, controller) in camera_query.iter_mut(){

        for ev in ev_motion.read() {
            let delta_x = ev.delta.x * std::f32::consts::PI * 2.0 * controller.sensitivity;
            let delta_y = ev.delta.y * std::f32::consts::PI * controller.sensitivity;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);

            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;

            update_camera(&mut transform, &controller);
        };

        ev_motion.clear();
    }
}

fn sync_orbit(
    mut camera_query: Query<(&mut CameraController, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>
){
    let Ok((mut camera, mut camera_transform)) = camera_query.get_single_mut() else { return };
    let Ok(player) = player_query.get_single() else { return };

    let delta = player.translation - camera.focus;

    if delta != Vec3::ZERO {
        camera.focus = player.translation;
        camera_transform.translation += delta;
    }
}

fn update_camera(transform: &mut Transform, controller: &CameraController){
    let matrix = Mat3::from_quat(transform.rotation);

    transform.translation =
        controller.focus + matrix.mul_vec3(Vec3::new(0.0, 0.0, controller.distance));
    transform.rotation.x =
        f32::clamp(transform.rotation.x, -controller.rotate_lock, controller.rotate_lock);
}
