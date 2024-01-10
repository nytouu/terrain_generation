use std::fmt::Debug;

use bevy::{prelude::*, pbr::wireframe::Wireframe};
use bevy_flycam::FlyCam;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, ComputedColliderShape}};
use rand::Rng;
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use futures_lite::future;

use super::generation::create_mesh;

const CHUNK_WORLD_SCALE: f32 = 512.0;
const CHUNK_WORLD_SIZE: f32 = 128.0;

#[derive(Event)]
pub struct ChunkEvent(ChunkBuilder);

pub struct ChunkBuilder {
    pub lod: usize,
    pub coords: Vec2
}

#[derive(Component)]
pub struct Chunk {
    pub mesh: Mesh,
    pub lod: usize,
    pub coords: Vec2
}

#[derive(Component)]
pub struct ChunkTask(Task<Chunk>);

impl Chunk {
    fn new(coords: Vec2, lod: usize) -> Chunk {
        Chunk {
            mesh: create_mesh(0.25, 0.2, lod, lod, coords),
            lod,
            coords
        }
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":Chunk({}):", self.coords)
    }
}

pub fn setup_chunks(
    mut ev_chunk: EventWriter<ChunkEvent>,
){
    ev_chunk.send(ChunkEvent(ChunkBuilder{
        lod: 16,
        coords: Vec2::new(0.0, 0.0)
    }));
}

pub fn handle_chunks_event(
    chunks: Query<&Chunk>,
    player_query: Query<&Transform, With<FlyCam>>,
    mut ev_chunk: EventWriter<ChunkEvent>,
){
    if let Ok(player_transform) = player_query.get_single() {
        let translation = player_transform.translation;
        let current_chunk = Vec2::new(
            (translation.x / CHUNK_WORLD_SIZE).round(),
            (translation.z / CHUNK_WORLD_SIZE).round()
        );
        let neighbors = get_neighbors(current_chunk);

        for neighbor in neighbors {
            let mut should_generate = true;
            for chunk in chunks.iter() {
                if chunk.coords == neighbor {
                    should_generate = false;
                }
            }
            if should_generate {
                ev_chunk.send(ChunkEvent(ChunkBuilder{
                    lod: 16,
                    coords: Vec2::new(neighbor.x, neighbor.y)
                }));
                info!("chunk event sent for {:?}", neighbor);
            }
        }

    }
}

pub fn spawn_chunk_task(
    mut commands: Commands,
    mut ev_chunk: EventReader<ChunkEvent>,
){
    let thread_pool = AsyncComputeTaskPool::get();

    for ev in ev_chunk.read() {
        let builder = &ev.0;
        let x = builder.coords.x;
        let y = builder.coords.y;

        let task = thread_pool.spawn(async move {
            Chunk::new(Vec2::new(x, y), 32)
        });

        commands.spawn(ChunkTask(task));
    }
}

pub fn handle_chunk_tasks(
    mut commands: Commands,
    mut chunk_tasks: Query<(Entity, &mut ChunkTask)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    let mut rng = rand::thread_rng();

    for (entity, mut task) in &mut chunk_tasks {
        if let Some(new_chunk) = block_on(future::poll_once(&mut task.0)) {
            let x = new_chunk.coords.x;
            let y = new_chunk.coords.y;

            // Add our new PbrBundle of components to our tagged entity
            commands.entity(entity).insert((
                PbrBundle {
                    mesh: meshes.add(new_chunk.mesh.clone()),
                    material: materials.add(Color::rgb(rng.gen(),rng.gen(), rng.gen()).into()),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * CHUNK_WORLD_SIZE, 0.0, y as f32 * CHUNK_WORLD_SIZE),
                        scale: Vec3::new(CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed,
                Collider::from_bevy_mesh(&new_chunk.mesh, &ComputedColliderShape::TriMesh).unwrap(),
                Wireframe,
            ));

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ChunkTask>();
        }
    }
}

pub fn remove_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &mut Chunk)>,
    player_query: Query<&Transform, With<FlyCam>>,
){
    if let Ok(player_transform) = player_query.get_single() {
        let translation = player_transform.translation;
        let current_chunk = Vec2::new(
            (translation.x / CHUNK_WORLD_SIZE).round(),
            (translation.z / CHUNK_WORLD_SIZE).round()
        );
        let neighbors = get_neighbors(current_chunk);

        for (entity, chunk) in chunks.iter() {
            let mut should_remove = true;

            for neighbor in neighbors.iter() {
                if neighbor == &chunk.coords {
                    should_remove = false;
                }
            }

            if should_remove {
                commands.entity(entity).despawn();
                info!("Despawned chunk {:?}", chunk.coords);
            }
        }
    }
}

fn get_neighbors(coords: Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(coords.x, coords.y + 1.0),
        Vec2::new(coords.x + 1.0, coords.y + 1.0),
        Vec2::new(coords.x + 1.0, coords.y),
        Vec2::new(coords.x, coords.y - 1.0),
        Vec2::new(coords.x - 1.0, coords.y - 1.0),
        Vec2::new(coords.x - 1.0, coords.y),
        Vec2::new(coords.x + 1.0, coords.y - 1.0),
        Vec2::new(coords.x - 1.0, coords.y + 1.0),
    ]
}
