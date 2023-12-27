use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}};

use crate::{postprocess::PostProcessSettings, player::Player};

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
    pub rotate_lock: f32,
    pub rotation: Vec3,

    pub focus: Vec3,
    pub distance: f32
}

fn setup_camera(mut commands: Commands){
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
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
            sensitivity: (0.173) / 100.0,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            focus: Vec3::ZERO,
            distance: 7.5,
        }
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn camera_zoom(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraController>
){
    let Ok(mut camera_controller) = camera_query.get_single_mut() else { return };

    for ev in ev_scroll.read() {
        camera_controller.distance -= ev.y / 6.0;
    }
}

fn camera_orbit(
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
){
    for (mut transform, controller) in camera_query.iter_mut(){
        for ev in ev_motion.read() {
            let delta_x = ev.delta.x / 1000.0 * std::f32::consts::PI * 2.0;
            let delta_y = ev.delta.y / 1000.0 * std::f32::consts::PI;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;

            let rot_matrix = Mat3::from_quat(transform.rotation);

            transform.translation =
                controller.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, controller.distance));

            transform.rotation.x =
                f32::clamp(transform.rotation.x, -controller.rotate_lock, controller.rotate_lock);
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
