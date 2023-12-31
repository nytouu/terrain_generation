use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (setup_player, setup_player_state))
            .add_systems(PreUpdate, check_grounded)
            .add_systems(Update, keyboard_movement);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    lerp_factor: f32,
    // air_friction: f32,
    // deadzone: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 10.0,
            lerp_factor: 0.95,
            // air_friction: 0.2,
            // deadzone: 0.20
        }
    }
}

#[derive(Resource)]
pub struct PlayerState {
    grounded: bool
}

fn setup_player_state(mut commands: Commands){
    commands.insert_resource(PlayerState {
        grounded: false
    })
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let player =
        (PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule{
                radius: 0.5,
                depth: 1.0,
                ..Default::default()
            })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..Default::default()
        }, 
        Player {
            speed: 20.0,
            ..Default::default()
        },

        // physics
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            custom_mass: Some(10.0),
            ..Default::default()
        },
        Collider::capsule_y(0.5, 0.5),
    );

    commands.spawn(player);
}

fn keyboard_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    state: Res<PlayerState>,
    mut player_query: Query<(&mut Transform, &mut KinematicCharacterController, &Player)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
){
    for (mut player_transform, mut player_controller, player) in player_query.iter_mut(){
        let camera: &Transform = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera : {}", e)).unwrap(),
        };
        let old_direction = player_transform.rotation;
        let mut direction = old_direction.clone().to_scaled_axis();
        let mut any = false;

        // handle keyboard
        if keys.pressed(KeyCode::Z){
            direction += camera.forward();
            any = true;
        }
        if keys.pressed(KeyCode::S){
            direction += camera.back();
            any = true;
        }
        if keys.pressed(KeyCode::Q){
            direction += camera.left();
            any = true;
        }
        if keys.pressed(KeyCode::D){
            direction += camera.right();
            any = true;
        }

        // gravity
        direction.y = -0.981;

        // apply direction
        direction = direction.normalize_or_zero();
        let movement = direction * player.speed * time.delta_seconds();
        player_controller.translation = Some(movement);

        // lerp rotate player towards direction
        if any {
            player_transform.rotation =
                Quat::from_rotation_y(direction.x.atan2(direction.z)).lerp(old_direction, player.lerp_factor);
        }
    }
}

fn check_grounded(
    controllers: Query<(Entity, &KinematicCharacterControllerOutput)>,
    mut state: ResMut<PlayerState>,
) {
    for (_entity, output) in controllers.iter() {
        // info!("{:?}", output.grounded);

        match output.grounded {
            true => state.grounded = true,
            false => state.grounded = false
        }
    }
}
