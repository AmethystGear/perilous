use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

use crate::point::Point;

pub fn verts_to_mesh(verts: &[Point<f32, 2>]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    set_mesh_attributes_according_to_verts(&mut mesh, verts);
    mesh
}

pub fn set_mesh_attributes_according_to_verts(mesh: &mut Mesh, verts: &[Point<f32, 2>]) {
    let num_verts = verts.len() as u32;
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        verts
            .iter()
            .map(|p| Vec3::new(p[0] as f32, p[1] as f32, 0.0))
            .collect::<Vec<_>>(),
    );
    mesh.insert_indices(Indices::U32((0..num_verts).collect()));
}
