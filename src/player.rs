use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_player)
           .add_systems(Update, player_movement);
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
            speed: 150.0,
            lerp_factor: 0.95
        }
    }
}

fn setup_player(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>
){
    let player = 
        // (PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        //     material: materials.add(Color::RED.into()),
        //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //     ..Default::default()
        // }, 
        (
        RigidBody::KinematicPositionBased
    );

        Player {
            speed: 50.0,
            ..Default::default()
        },

    commands.spawn(player)
        .insert(KinematicCharacterController {
            custom_mass: Some(10.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(0.5, 0.5 , 0.5))
        .insert(Name::new("Player"))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 10.0, 0.0)));
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut KinematicCharacterController, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>
){
    for (mut player_transform, mut player_controller, player_speed) in player_query.iter_mut(){
        let camera: &Transform = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera : {}", e)).unwrap(),
        };
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::Z){
            direction += camera.forward()
        }
        if keys.pressed(KeyCode::S){
            direction += camera.back()
        }
        if keys.pressed(KeyCode::Q){
            direction += camera.left()
        }
        if keys.pressed(KeyCode::D){
            direction += camera.right()
        }

        direction.y = -5.0;
        println!("{}", direction);

        direction = direction.normalize_or_zero();
        let movement = direction * player.speed * time.delta_seconds();
        player_controller.translation = Some(movement);
    }
}
