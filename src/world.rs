use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(Color::BLUE),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 10.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Restitution::new(0.5),
        Collider::ball(1.0),
    ));
}
