use bevy::prelude::*;

pub mod chunk;
pub mod mesh;
pub mod noise;

use self::chunk::*;

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_chunks);
        app.add_event::<ChunkEvent>();
        app.add_systems(FixedUpdate, (
            handle_chunks_event,
            spawn_replace_task,
            spawn_chunk_task,
            handle_replace_tasks,
            handle_chunk_tasks,
            remove_chunks
        ).chain());
    }
}
