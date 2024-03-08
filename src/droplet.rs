use bevy::prelude::*;

use crate::marching_squares::{marching_squares, matrix::Matrix, point::Point, tiles::Tiles};

#[derive(Component)]
struct Droplet {
    pos: Vec<(Vec2, f32)>,
}

/// Calculates droplet geometry using a grid-based marching squares approach.
/// posns - a slice of posn/size tuples
/// res - the resolution of the grid
/// scale - distance between points on the grid
fn calculate_droplet_geometry(posns: &[(Vec2, f32)], res : [usize; 2], scale: f64) -> Vec<Point<f64, 2>> {
    let mut elems: Vec<i8> = vec![0; res[0] * res[1]];
    for y in 0..res[1] {
        for x in 0..res[0] {
            let idx = y * res[0] + x;
            let elempos = Vec2::new(
                x as f32 - res[0] as f32 / 2.0,
                y as f32 - res[1] as f32 / 2.0,
            ) * scale as f32;
            for (pos, v) in posns {
                let diff = elempos - (*pos - posns[0].0);
                elems[idx] = elems[idx]
                    .saturating_add((v / diff.length_squared() - 0.5 * i8::MAX as f32) as i8);
            }
        }
    }
    let mat = Matrix::new(res, &elems);
    let tiles: Tiles<i8> = Tiles::new(mat, scale);
    let (verts, _) = marching_squares(&tiles);
    verts
}



