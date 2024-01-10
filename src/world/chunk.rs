use bevy::{prelude::*, pbr::wireframe::Wireframe};
use bevy_flycam::FlyCam;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, ComputedColliderShape}};
use rand::Rng;

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

impl Chunk {
    fn new(coords: Vec2, lod: usize) -> Chunk {
        Chunk {
            mesh: create_mesh(0.25, 0.2, lod, lod, coords),
            lod,
            coords
        }
    }
}

#[derive(Resource)]
pub struct ChunkData {
    pub chunklist: Vec<Chunk>
}

pub fn setup_chunks(
    mut ev_chunk: EventWriter<ChunkEvent>,
){
    ev_chunk.send(ChunkEvent(ChunkBuilder{
        lod: 16,
        coords: Vec2::new(0.0, 0.0)
    }));
    // let mut rng = rand::thread_rng();

    // for x in 0..3 {
    //     for y in 0..3 {
    //         let x = x as f32;
    //         let y = y as f32;
    //
    //         let starting_chunk = Chunk::new(Vec2::new(x * 2.0, y * 2.0), 32);
    //         commands.spawn((
    //             PbrBundle {
    //                 mesh: meshes.add(starting_chunk.mesh.clone()),
    //                 material: materials.add(Color::rgb(rng.gen(), rng.gen(), rng.gen()).into()),
    //                 transform: Transform {
    //                     translation: Vec3::new(x as f32 * CHUNK_WORLD_SIZE, 0.0, y as f32 * CHUNK_WORLD_SIZE),
    //                     scale: Vec3::new(CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE),
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             RigidBody::Fixed,
    //             Collider::from_bevy_mesh(&starting_chunk.mesh, &ComputedColliderShape::TriMesh).unwrap(),
    //             Wireframe,
    //         ));
    //         chunks.chunklist.insert(0, starting_chunk);
    //     }
    // }
    // info!("successfully spawned chunk {}", chunks.);
}

pub fn handle_chunks(
    keys: Res<Input<KeyCode>>,
    chunks: ResMut<ChunkData>,
    player_query: Query<&Transform, With<FlyCam>>,
    mut ev_chunk: EventWriter<ChunkEvent>,
){
    if let Ok(player_transform) = player_query.get_single() {
        let translation = player_transform.translation;
        let current_chunk = Vec2::new(
            (translation.x / CHUNK_WORLD_SIZE).round(),
            (translation.z / CHUNK_WORLD_SIZE).round()
        );

        for chunk in chunks.chunklist.iter() {
            if chunk.coords != current_chunk && keys.just_pressed(KeyCode::A) {
                ev_chunk.send(ChunkEvent(ChunkBuilder{
                    lod: 16,
                    coords: Vec2::new(current_chunk.x, current_chunk.y)
                }));
                info!("chunk event sent for {:?}", current_chunk);
            }
        }

        // info!("{}", current_chunk);
    } else { panic!("Couldn't find player transform") };
}

pub fn build_chunks(
    mut commands: Commands,
    mut ev_chunk: EventReader<ChunkEvent>,
    mut chunks: ResMut<ChunkData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut rng = rand::thread_rng();

    for ev in ev_chunk.read() {
        info!("chunk event read");

        let builder = &ev.0;
        let x = builder.coords.x;
        let y = builder.coords.y;

        let new_chunk = Chunk::new(Vec2::new(x * 2.0, y * 2.0), 32);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(new_chunk.mesh.clone()),
                material: materials.add(Color::rgb(rng.gen(), rng.gen(), rng.gen()).into()),
                transform: Transform {
                    translation: Vec3::new(x * CHUNK_WORLD_SIZE, 0.0, y * CHUNK_WORLD_SIZE),
                    scale: Vec3::new(CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE, CHUNK_WORLD_SCALE),
                    ..default()
                },
                ..default()
            },
            RigidBody::Fixed,
            Collider::from_bevy_mesh(&new_chunk.mesh, &ComputedColliderShape::TriMesh).unwrap(),
            Wireframe,
        ));

        let index = chunks.chunklist.len();
        chunks.chunklist.insert(index, new_chunk);
    }
}
