use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod noise;
pub mod generation;
pub mod chunk;

use self::chunk::{setup_chunks, handle_chunks_event, ChunkData, Chunk, ChunkEvent, spawn_chunk_task, handle_chunk_tasks};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App){
        app.insert_resource(ChunkData(Vec::<Chunk>::new()))
            .add_systems(Startup, (
                setup_world,
                setup_chunks
            ));
        app.add_event::<ChunkEvent>();
        app.add_systems(Update, (
            handle_chunks_event,
            spawn_chunk_task,
            handle_chunk_tasks
        ));
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn((PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform {
                translation: Vec3 { x: 0.0, y: 10.0, z: 0.0 },
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Restitution::new(0.5),
        Collider::ball(1.0),
    ));
}
