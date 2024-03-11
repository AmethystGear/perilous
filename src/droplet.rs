

use std::ops::Add;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{marching_squares::{marching_squares, matrix::Matrix, tiles::Tiles}, mesh::{set_mesh_attributes_according_to_verts, verts_to_mesh}, point::Point};

#[derive(Component)]
pub struct Droplet {
    pub posns: Vec<(Vec2, f32)>,
    pub max_posns_len: usize,
}

/// Calculates droplet geometry using a grid-based marching squares approach
fn calculate_droplet_geometry(
    posns: &[(Vec2, f32)],
    res: [usize; 2],
    scale: f64,
    radius: f32,
    decay: f32,
) -> Vec<Point<f32, 2>> {
    let mut elems = vec![0.0; res[0] * res[1]];
    for y in 0..res[1] {
        for x in 0..res[0] {
            let idx = y * res[0] + x;
            let elempos = Vec2::new(
                x as f32 - res[0] as f32 / 2.0,
                y as f32 - res[1] as f32 / 2.0,
            ) * scale as f32;
            let mut r = radius;
            let mut sum = 0.0;
            for (pos, f) in posns {
                let diff = elempos - (*pos - posns[0].0);
                sum += (1.0 - r * r * f * f / diff.length_squared()).max(-10000.0);
                r *= decay;
            }
            elems[idx] = sum / posns.len() as f32;
        }
    }
    let mat = Matrix::new(res, &elems);
    let tiles: Tiles<f32> = Tiles::new(mat, scale);
    let (verts, _) = marching_squares(&tiles);
    verts
}

pub fn move_droplet(
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
    droplet.posns.insert(
        0,
        (
            droplet_transform.translation.xy().into(),
            ((time.elapsed_seconds_f64() * 10.).sin() * 0.2 + 1.0) as f32,
        ),
    );
    if droplet.posns.len() > droplet.max_posns_len {
        droplet.posns.pop();
    }
    droplet_transform.translation += Vec3::new(dir.x, dir.y, 0.0) * time.delta_seconds() * 150.0;

    let geom = calculate_droplet_geometry(&droplet.posns, [75, 75], 100. / 75., 20., 0.925);
    if let Some(mesh) = meshes.get_mut(droplet_mesh_handle.0.id()) {
        set_mesh_attributes_according_to_verts(mesh, &geom);
    }
}

pub fn setup_droplet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let droplet = Droplet {
        posns: Vec::new(),
        max_posns_len: 20,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            // start our droplet out with an empty mesh, it'll get updated next frame anyways
            mesh: meshes.add(verts_to_mesh(&[])).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        },
        droplet,
    ));
}