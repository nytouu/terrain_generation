use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, keyboard_movement);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    lerp_factor: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 10.0,
            lerp_factor: 0.95,
        }
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 1.0,
                ..default()
            })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(10.0, 10.0, 0.0),
            ..default()
        },
        Player {
            speed: 20.0,
            ..default()
        },
        // physics
        RigidBody::KinematicVelocityBased,
        LockedAxes::ROTATION_LOCKED,
        GravityScale(10.5),
        Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        },
        KinematicCharacterController {
            custom_mass: Some(500.0),
            ..default()
        },
        Collider::capsule_y(0.5, 0.5),
    );

    commands.spawn(player);
}

fn keyboard_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        &mut Velocity,
        &Player,
    )>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, mut player_controller, mut player_velocity, player) in
        player_query.iter_mut()
    {
        let camera: &Transform = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera : {}", e)).unwrap(),
        };
        let old_direction = player_transform.rotation;
        let mut direction = old_direction.clone().to_scaled_axis();
        let mut any = false;

        let mut velocity = Vec3::ZERO;

        // handle keyboard
        for key in keys.get_pressed() {
            any = true;
            match key {
                KeyCode::Z => velocity += camera.forward(),
                KeyCode::S => velocity += camera.back(),
                KeyCode::Q => velocity += camera.left(),
                KeyCode::D => velocity += camera.right(),
                _ => (),
            }
        }
        velocity = velocity.normalize_or_zero();

        player_velocity.linvel.x = 0.0;
        player_velocity.linvel.y = -9.0;
        player_velocity.linvel.z = 0.0;
        player_velocity.linvel += velocity * player.speed * time.delta_seconds() * 100.0;

        // lerp rotate player towards direction
        direction = direction.normalize_or_zero();
        if any {
            player_transform.rotation = Quat::from_rotation_y(direction.x.atan2(direction.z))
                .lerp(old_direction, player.lerp_factor);
        }
    }
}
