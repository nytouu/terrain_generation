use bevy::{prelude::*, pbr::wireframe::Wireframe};
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, ComputedColliderShape}};
use rand::Rng;

use super::generation::create_mesh;

const CHUNK_WORLD_SIZE: f32 = 512.0;

#[derive(Component)]
pub struct Chunk {
    pub mesh: Mesh,
    pub lod: usize,
}

impl Chunk {
    fn new(coords: Vec2, lod: usize) -> Chunk {
        Chunk {
            mesh: create_mesh(0.25, 0.2, lod, lod, coords),
            lod
        }
    }
}

#[derive(Resource)]
pub struct ChunkData {
    pub chunklist: Vec<Chunk>
}

pub fn setup_chunks(
    mut commands: Commands,
    mut chunks: ResMut<ChunkData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut rng = rand::thread_rng();

    for x in 0..3 {
        for y in 0..3 {
            let x = x as f32;
            let y = y as f32;

            let starting_chunk = Chunk::new(Vec2::new(x * 2.0, y * 2.0), 32);
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(starting_chunk.mesh.clone()),
                    material: materials.add(Color::rgb(rng.gen(), rng.gen(), rng.gen()).into()),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * 128.0, 0.0, y as f32 * 128.0),
                        scale: Vec3::new(CHUNK_WORLD_SIZE, CHUNK_WORLD_SIZE, CHUNK_WORLD_SIZE),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed,
                Collider::from_bevy_mesh(&starting_chunk.mesh, &ComputedColliderShape::TriMesh).unwrap(),
                Wireframe,
            ));
            chunks.chunklist.insert(0, starting_chunk);
        }

    }

    // info!("successfully spawned chunk {}", chunks.);
}

pub fn handle_chunks(
    commands: Commands,
    chunks: ResMut<ChunkData>,
    // player_query: Query<Transform, With<FlyCam>>,
){
}

fn spawn_chunk(
    coords: Vec2,
    lod: usize,
){
    todo!();
}
