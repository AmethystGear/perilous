pub mod point;
pub mod tiles;
pub mod matrix;

use lazy_static::lazy_static;
use num_traits::{AsPrimitive, One, SaturatingAdd, Signed, Zero};

use {
    point::{Numeric, Point},
    tiles::Tiles,
};

const CORNERS: [Point<i32, 2>; 4] = {
    [
        Point::new([0, 0]),
        Point::new([1, 0]),
        Point::new([1, 1]),
        Point::new([0, 1]),
    ]
};

lazy_static! {
    ///
    /// tile points layout:
    ///   6----5----4
    ///   |         |
    ///   7         3
    ///   |         |
    ///   0----1----2
    ///
    static ref TRIANGLE_MAPPINGS: [[Vec<usize>; 16]; 2] = {
        [
            // default ruleset, use if there are any negative densities
            [
                vec![],
                vec![7, 1, 0],
                vec![3, 2, 1],
                vec![7, 2, 0, 7, 3, 2],
                vec![5, 4, 3],
                vec![7, 1, 0, 5, 4, 3, 7, 3, 1, 7, 5, 3],
                vec![1, 4, 2, 1, 5, 4],
                vec![2, 0, 7, 5, 2, 7, 4, 2, 5],
                vec![7, 6, 5],
                vec![0, 5, 1, 0, 6, 5],
                vec![3, 2, 1, 7, 6, 5, 7, 3, 1, 7, 5, 3],
                vec![6, 5, 0, 5, 3, 0, 3, 2, 0],
                vec![6, 3, 7, 6, 4, 3],
                vec![6, 4, 3, 6, 3, 1, 6, 1, 0],
                vec![6, 4, 7, 4, 1, 7, 4, 2, 1],
                vec![0, 6, 8, 0, 8, 2, 4, 8, 2, 6, 8, 4],
            ],
            // zero and positive density only ruleset (for 90 degree corners)
            [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![0, 6, 8, 0, 8, 2, 4, 8, 2, 6, 8, 4],
            ]
        ]
    };
}

/// for marching squares, each combination of densities creates a different combination of triangles
/// and edges. This takes the 4 densities of a tile at the provided location (every tile is a square, so it has 4 corners)
/// and finds the correct ruleset and map index within that ruleset to use to get the correct edge and triangle data.
/// Use the returned (ruleset, map_id) tuple to index `TRIANGLE_MAPPINGS` and `EDGE_MAPPINGS` for the triangle
/// and edge data respectively (e.g. `TRIANGLE_MAPPINGS[ruleset][map_id]`).
fn get_ruleset_and_map_id<T: Numeric<T> + Default + SaturatingAdd + One + Signed>(
    loc: Point<i32, 2>,
    tiles: &Tiles<T>,
) -> (usize, usize) {
    let positions = CORNERS.map(|x| loc + x);
    let densities = positions.map(|x| tiles.get(x));

    let ruleset = densities
        .iter()
        .all(|x| x.saturating_add(&T::one()).is_positive()) as usize;
    let map_id = densities.iter().enumerate().fold(0, |val, (i, density)| {
        val + ((!density.is_positive() as usize) << i)
    });
    (ruleset, map_id)
}

/// Based on the density of the two corners of a tile, find the point that we
/// need to use for marching squares. This point can be represented with a single f64 value
/// that we return, which is the interpolation float between the two points.
/// (We use this value to lerp between the first and second points).
fn get_density_proportion<
    T: Numeric<T> + Default + AsPrimitive<i32> + Eq + PartialEq + Zero + AsPrimitive<f64>,
>(
    loc: Point<i32, 2>,
    corner_indices: [usize; 2],
    tiles: &Tiles<T>,
) -> (f64, [usize; 2]) {
    let tile_loc = corner_indices.map(|x| loc + CORNERS[x]);
    let densities = tile_loc.map(|x| tiles.get(x));

    let diff = {
        let densities: [i32; 2] = [densities[0], densities[1]].map(|density| density.as_());
        densities[0] - densities[1]
    };

    if densities[1] == T::zero() {
        (1.0, [corner_indices[1], corner_indices[1]])
    } else if diff == 0 || densities[0] == T::zero() {
        (0.0, [corner_indices[0], corner_indices[0]])
    } else {
        let d: f64 = densities[0].as_();
        (d.abs() / (diff as f64), corner_indices)
    }
}

/// converts an index from 0 - 7 inclusive into
/// two indices from 0 - 3 inclusive, which represent the point(s)
/// on the tile that need to be interpolated to get the true point.
/// if index is even, both of the returned indices will be the same,
/// otherwise, the second index will be one greater than the first.
fn index_to_corner_indices(index: usize) -> [usize; 2] {
    [index / 2, ((index + 1) / 2) % 4]
}

/// modified version of the marching squares algorithm.
/// this modified algorithm allows you to make 90 degree corners along
/// nodes, which isn't possible with the original marching squares algorithm.
/// returns (vertices_to_render, collision_vertices)
pub fn marching_squares<
    T: Numeric<T>
        + Default
        + AsPrimitive<i32>
        + Eq
        + PartialEq
        + Zero
        + AsPrimitive<f64>
        + SaturatingAdd
        + One
        + Signed,
>(
    tiles: &Tiles<T>,
) -> (Vec<Point<f64, 2>>, Vec<Point<f64, 2>>) {
    let corners = CORNERS.map(|p| Point::new([p[0] as f64, p[1] as f64]));
    let mut collision_vertices = Vec::new();
    let mut vertices = Vec::new();
    for y in -2..(tiles.dimension()[1] + 1) as i32 {
        for x in -2..(tiles.dimension()[0] + 1) as i32 {
            let loc = [x, y].into();
            let (ruleset, map_id) = get_ruleset_and_map_id(loc, tiles);
            let tile_location: Point<f64, 2> = (loc,).into();
            let tile_location = tile_location * tiles.dist_between_nodes();
            for point in &TRIANGLE_MAPPINGS[ruleset][map_id] {
                let rel_loc = if *point == 8 {
                    corners[0].lerp(corners[2], 0.5)
                } else {
                    let corner_indices = index_to_corner_indices(*point);
                    let (prop, corner_indices) = get_density_proportion(loc, corner_indices, tiles);
                    corners[corner_indices[0]].lerp(corners[corner_indices[1]], prop)
                };

                let neighbors = [
                    [0, 0].into(),
                    [-1, 0].into(),
                    [0, -1].into(),
                    [1, 0].into(),
                    [0, 1].into(),
                ];

                let l = rel_loc * tiles.dist_between_nodes() + tile_location;
                let empty_nearby = neighbors.into_iter().any(|x| {
                    let (ruleset, map_id) = get_ruleset_and_map_id(loc + x, tiles);
                    ruleset == 1 && map_id != 15
                });
                if (ruleset == 1 && map_id == 15 && empty_nearby) || (ruleset == 0 && map_id != 15)
                {
                    collision_vertices.push(l);
                }
                vertices.push(l);
            }
        }
    }
    (vertices, collision_vertices)
}
