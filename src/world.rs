use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let plane = PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
        material: materials.add(Color::CYAN.into()),
        ..Default::default()
    };

    let ball = PbrBundle {
        mesh: meshes.add(shape::UVSphere::default().into()),
        material: materials.add(Color::GREEN.into()),
        ..Default::default()
    };

    commands.spawn(plane)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 0.02, 500.0));
    commands.spawn(ball)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3 { x: 3.0, y: 10.0, z: 2.0 },
                ..Default::default()
            }, 
            ..Default::default()
        });
}
