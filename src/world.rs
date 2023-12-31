use bevy::{prelude::*, pbr::wireframe::Wireframe};
use bevy_rapier3d::prelude::*;

use noise::Perlin;
use rand::Rng;

use crate::mesh::create_mesh;

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
    let mut rng = rand::thread_rng();

    let mesh = create_mesh::<Perlin>(rng.gen::<u32>());
    let plane = (PbrBundle {
            mesh: meshes.add(mesh.clone()),
            // mesh: meshes.add(shape::Plane::from_size(20.0).into()),
            material: materials.add(Color::CYAN.into()),
            transform: Transform {
                scale: Vec3::new(150.0, 150.0, 150.0),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
        // Wireframe,
        // Collider::cuboid(1.0, 0.02, 1.0)
    );

    let ball = (PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform {
                translation: Vec3 { x: 3.0, y: 10.0, z: 2.0 },
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Dynamic,
        Restitution::new(0.5),
        Collider::ball(1.0),
    );

    commands.spawn(plane);
    commands.spawn(ball);
}
