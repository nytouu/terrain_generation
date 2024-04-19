use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy::{pbr::wireframe::Wireframe, prelude::*};
use bevy_flycam::FlyCam;
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ComputedColliderShape},
};
use futures_lite::future;

use super::mesh::create_mesh;

const CHUNK_WORLD_SCALE: f32 = 512.0;
const CHUNK_WORLD_SIZE: f32 = 112.0;

const FAR_LOD: usize = 16;
const NORMAL_LOD: usize = 32;

const TERRAIN_ALPHA: f32 = 1.0;

const RENDER_DISTANCE: i32 = 6;

const HEIGHT_INTENSITY: f32 = 0.2;
const MAP_SIZE: f64 = 0.25;

pub struct ChunkDescriptor {
    pub lod: usize,
    pub coords: Vec2,
}

#[derive(Component)]
pub struct Chunk {
    pub mesh: Mesh,
    pub lod: usize,
    pub coords: Vec2,
}

#[derive(Component)]
pub struct ChunkTask {
    pub task: Task<Chunk>,
    pub descriptor: ChunkDescriptor,
}

#[derive(Component)]
pub struct ReplaceTask {
    pub task: Task<Chunk>,
    pub descriptor: ChunkDescriptor,
}

impl Chunk {
    fn new(coords: Vec2, lod: usize) -> Chunk {
        Chunk {
            mesh: create_mesh(MAP_SIZE, HEIGHT_INTENSITY, lod, lod, coords),
            lod,
            coords,
        }
    }
}

pub fn setup_chunks(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { Chunk::new(Vec2::new(0.0, 0.0), NORMAL_LOD) });

    commands.spawn(ChunkTask {
        task,
        descriptor: ChunkDescriptor {
            lod: NORMAL_LOD,
            coords: Vec2::new(0.0, 0.0),
        },
    });
}

pub fn handle_new_chunks(
    mut commands: Commands,
    chunks: Query<&Chunk>,
    tasks: Query<&ChunkTask>,
    player_query: Query<&Transform, With<FlyCam>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let thread_pool = AsyncComputeTaskPool::get();

        let current_chunk = get_player_chunk(player_transform.translation);
        let neighbors = get_neighbors(current_chunk, RENDER_DISTANCE);

        for (neighbor, lod) in &neighbors {
            let mut already_tasked = false;
            for task in tasks.iter() {
                for (neighbor, _) in &neighbors {
                    if task.descriptor.coords == *neighbor {
                        already_tasked = true;
                    }
                }
            }

            let mut already_generated = false;
            for chunk in chunks.iter() {
                if chunk.coords == *neighbor {
                    already_generated = true;
                }
            }

            if !already_generated && !already_tasked {
                let x = neighbor.x;
                let y = neighbor.y;
                let lod = *lod;

                let task = thread_pool.spawn(async move { Chunk::new(Vec2::new(x, y), lod) });

                commands.spawn(ChunkTask {
                    task,
                    descriptor: ChunkDescriptor {
                        lod,
                        coords: Vec2::new(x, y),
                    },
                });
            }
        }
    }
}

pub fn handle_chunk_tasks(
    mut commands: Commands,
    mut chunk_tasks: Query<(Entity, &mut ChunkTask)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, mut task) in &mut chunk_tasks {
        if let Some(new_chunk) = block_on(future::poll_once(&mut task.task)) {
            let x = new_chunk.coords.x;
            let y = new_chunk.coords.y;

            // Add our new PbrBundle of components to our tagged entity
            commands.entity(entity).insert((
                PbrBundle {
                    mesh: meshes.add(new_chunk.mesh.clone()),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, TERRAIN_ALPHA)),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * CHUNK_WORLD_SIZE,
                            0.0,
                            y as f32 * CHUNK_WORLD_SIZE,
                        ),
                        scale: Vec3::new(CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed,
                Collider::from_bevy_mesh(&new_chunk.mesh, &ComputedColliderShape::TriMesh).unwrap(),
                Wireframe,
                new_chunk,
            ));

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ChunkTask>();
        }
    }
}

pub fn remove_chunks(
    mut commands: Commands,
    chunks: Query<(Entity, &Chunk)>,
    player_query: Query<&Transform, With<FlyCam>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let current_chunk = get_player_chunk(player_transform.translation);
        let neighbors = get_neighbors(current_chunk, RENDER_DISTANCE);

        for (entity, chunk) in chunks.iter() {
            let mut should_remove = true;

            for (neighbor, _) in &neighbors {
                if current_chunk == chunk.coords || neighbor == &chunk.coords {
                    should_remove = false;
                }
            }
            if should_remove {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn spawn_replace_task(
    mut commands: Commands,
    chunks: Query<(Entity, &Chunk, Option<&ReplaceTask>)>,
    player_query: Query<&Transform, With<FlyCam>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let thread_pool = AsyncComputeTaskPool::get();

        let current_chunk = get_player_chunk(player_transform.translation);
        let neighbors = get_neighbors(current_chunk, RENDER_DISTANCE);

        for (entity, chunk, task) in chunks.iter() {
            if task.is_none() {
                for (neighbor, lod) in &neighbors {
                    if neighbor == &chunk.coords && lod != &chunk.lod {
                        let x = neighbor.x;
                        let y = neighbor.y;
                        let lod = *lod;

                        let task =
                            thread_pool.spawn(async move { Chunk::new(Vec2::new(x, y), lod) });

                        commands.entity(entity).insert(ReplaceTask {
                            task,
                            descriptor: ChunkDescriptor {
                                lod,
                                coords: Vec2::new(x, y),
                            },
                        });
                    }
                }
            }
        }
    }
}

pub fn handle_replace_tasks(
    mut commands: Commands,
    mut replace_tasks: Query<(Entity, &mut ReplaceTask)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, mut task) in &mut replace_tasks {
        if let Some(replacing_chunk) = block_on(future::poll_once(&mut task.task)) {
            let x = replacing_chunk.coords.x;
            let y = replacing_chunk.coords.y;

            commands.entity(entity).despawn();

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(replacing_chunk.mesh.clone()),
                    material: materials.add(Color::rgba(1.0, 1.0, 1.0, TERRAIN_ALPHA)),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * CHUNK_WORLD_SIZE,
                            0.0,
                            y as f32 * CHUNK_WORLD_SIZE,
                        ),
                        scale: Vec3::new(CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Fixed,
                Collider::from_bevy_mesh(&replacing_chunk.mesh, &ComputedColliderShape::TriMesh)
                    .unwrap(),
                Wireframe,
                replacing_chunk,
            ));
        }
    }
}

fn get_neighbors(coords: Vec2, mut radius: i32) -> Vec<(Vec2, usize)> {
    let mut neighbors = Vec::<(Vec2, usize)>::new();
    if radius <= 0 {
        radius = 1
    };

    let start = -radius;
    let end = radius + 1;

    for x in start..end {
        for y in start..end {
            // current chunk isn't a neighbor
            if x != 0 || y != 0 {
                // closest chunk have higher lod
                if (x >= -1 && x <= 1) && (y >= -1 && y <= 1) {
                    neighbors.push((
                        Vec2::new(coords.x + x as f32, coords.y + y as f32),
                        NORMAL_LOD,
                    ));
                } else {
                    neighbors.push((Vec2::new(coords.x + x as f32, coords.y + y as f32), FAR_LOD));
                }
            }
        }
    }

    neighbors
}

fn get_player_chunk(player_translation: Vec3) -> Vec2 {
    Vec2::new(
        (player_translation.x / CHUNK_WORLD_SIZE).round(),
        (player_translation.z / CHUNK_WORLD_SIZE).round(),
    )
}
