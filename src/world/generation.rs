use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use super::noise::generate_noise_map;

// create_mesh function taken from : https://gitlab.lejondahl.com/bevy/bevy_holo
pub fn create_mesh(
    // seed: u32,
    size: f64,
    intensity: f32,
    width: usize,
    depth: usize,
    chunk: Vec2,
) -> Mesh {
    let extent: f64 = size;
    let intensity = intensity;
    let width: usize = width;
    let depth: usize = depth;

    // Create noisemap
    let noisemap = generate_noise_map(extent, width, depth, chunk);

    let vertices_count: usize = (width + 1) * (depth + 1);
    let triangle_count: usize = width * depth * 2 * 3;

    // Cast
    let (width_u32, depth_u32) = (width as u32, depth as u32);
    let (width_f32, depth_f32) = (width as f32, depth as f32);
    let extent_f32 = extent as f32;

    // Defining vertices
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);

    for d in 0..=width {
        for w in 0..=depth {
            let (w_f32, d_f32) = (w as f32, d as f32);

            let pos = [
                (w_f32 - width_f32 / 2.) * extent_f32 / width_f32,
                (noisemap.get_value(w, d) as f32) * intensity,
                (d_f32 - depth_f32 / 2.) * extent_f32 / depth_f32,
            ];
            positions.push(pos);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([w_f32 / width_f32, d_f32 / depth_f32]);
        }
    }

    // Defining triangles
    let mut triangles: Vec<u32> = Vec::with_capacity(triangle_count);

    for d in 0..depth_u32 {
        for w in 0..width_u32 {
            // First tringle
            triangles.push((d * (width_u32 + 1)) + w);
            triangles.push(((d + 1) * (width_u32 + 1)) + w);
            triangles.push(((d + 1) * (width_u32 + 1)) + w + 1);
            // Second triangle
            triangles.push((d * (width_u32 + 1)) + w);
            triangles.push(((d + 1) * (width_u32 + 1)) + w + 1);
            triangles.push((d * (width_u32 + 1)) + w + 1);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangles)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}