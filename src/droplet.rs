use std::ops::Neg;

use bevy::prelude::*;

use crate::marching_squares::{marching_squares, matrix::Matrix, point::Point, tiles::Tiles};

#[derive(Component)]
pub struct Droplet {
    pub posns: Vec<(Vec2, f32)>,
    pub max_posns_len: usize,
}

/// Calculates droplet geometry using a grid-based marching squares approach.
/// posns - a slice of posn/size tuples
/// res - the resolution of the grid
/// scale - distance between points on the grid
pub fn calculate_droplet_geometry(
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
            let mut x: f32 = 0.0;
            for (pos, f) in posns {
                let diff = elempos - (*pos - posns[0].0);
                let rv = r * f;
                sum += (1.0 - rv * rv / diff.length_squared()).max(-10000.0);
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
