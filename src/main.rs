use std::ops::Add;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use droplet::{calculate_droplet_geometry, Droplet};
use marching_squares::point::Point;

mod droplet;
mod marching_squares;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_droplet))
        .add_systems(FixedUpdate, (move_droplet,))
        .run();
}

fn move_droplet(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut droplet: Query<(&mut Droplet, &mut Transform, &Mesh2dHandle)>,
) {
    let dir = [
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyA, -Vec2::X),
        (KeyCode::KeyS, -Vec2::Y),
        (KeyCode::KeyD, Vec2::X),
    ]
    .into_iter()
    .filter(|(key, _)| keys.pressed(*key))
    .map(|(_, dir)| dir)
    .fold(Vec2::ZERO, Vec2::add)
    .normalize_or_zero();

    let (mut droplet, mut droplet_transform, droplet_mesh_handle) = droplet.single_mut();
    droplet
        .posns
        .insert(0, (droplet_transform.translation.xy().into(), ((time.elapsed_seconds_f64()* 10.).sin() * 0.2 + 1.0) as f32));
    if droplet.posns.len() > droplet.max_posns_len {
        droplet.posns.pop();
    }
    droplet_transform.translation += Vec3::new(dir.x, dir.y, 0.0) * time.delta_seconds() * 150.0;

    let geom = calculate_droplet_geometry(
        &droplet.posns,
        [100, 100],
        1.,
        20.,
        0.925,
    );
    if let Some(mesh) = meshes.get_mut(droplet_mesh_handle.0.id()) {
        set_mesh_attributes_per_verts(mesh, &geom);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_droplet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let droplet = Droplet {
        posns: Vec::from([(Vec2::ZERO, 1.0)]),
        max_posns_len: 20,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(verts_to_mesh(&[]))
                .into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        },
        droplet,
    ));
}

pub fn verts_to_mesh(verts: &[Point<f32, 2>]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    set_mesh_attributes_per_verts(&mut mesh, verts);
    mesh
}

pub fn set_mesh_attributes_per_verts(mesh : &mut Mesh, verts: &[Point<f32, 2>]) {
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
