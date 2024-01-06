use bevy::{prelude::*, pbr::wireframe::Wireframe};
use bevy_rapier3d::prelude::*;
// use bevy_procedural_grass::prelude::*;

pub mod noise;
pub mod generation;

use self::generation::create_mesh;

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
    // let mut rng = rand::thread_rng();

    // let mesh = create_mesh(0.3, 0.2, 64, 64);
    let mesh = create_mesh(1.0, 0.2, 256, 256);
    // let mesh = create_mesh::<Perlin>(50.0, 0.4, 32, 32);
    let _plane = commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.clone()),
            // mesh: meshes.add(shape::Plane::from_size(20.0).into()),
            material: materials.add(Color::CYAN.into()),
            transform: Transform {
                scale: Vec3::new(512.0, 512.0, 512.0),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
        Wireframe,
        // Collider::cuboid(1.0, 0.02, 1.0)
    )).id();

    // spawn grass
    // commands.spawn(GrassBundle {
    //     mesh: meshes.add(GrassMesh::mesh(7)), // how many segments you want in the mesh (no. of verts = segments * 2 + 1)
    //     grass: Grass {
    //         entity: Some(plane.clone()), // set entity that grass will generate on top of.
    //         ..default()
    //     },
    //     lod: GrassLODMesh::new(meshes.add(GrassMesh::mesh(3))), // optional: enables LOD
    //     ..default()
    // });

    let ball = (PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform {
                translation: Vec3 { x: 3.0, y: 10.0, z: 2.0 },
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Restitution::new(0.5),
        Collider::ball(1.0),
    );

    commands.spawn(ball);
}
