use super::{matrix::Matrix, point::{Numeric, Point}};

/// tilemap which returns a default density for
/// indices outside it's range.
pub struct Tiles<'a, T> where T : Numeric<T> + Default {
    densities: Matrix<'a, T, 2>,
    dist_between_nodes: f64,
}

impl<'a, T> Tiles<'a, T> where T : Numeric<T> + Default {
    pub fn new(densities: Matrix<'a, T, 2>, dist_between_nodes: f64) -> Self {
        Self {
            densities,
            dist_between_nodes,
        }
    }

    pub fn get(&self, loc: Point<i32, 2>) -> T {
        match [loc[0], loc[1]].map(|x| usize::try_from(x)) {
            [Ok(x), Ok(y)] if x < self.densities.dim()[0] && y < self.densities.dim()[1] => {
                self.densities.get([x, y])
            }
            _ => T::default(),
        }
    }

    pub fn dimension(&self) -> [usize; 2] {
        self.densities.dim()
    }

    pub fn dist_between_nodes(&self) -> f64 {
        self.dist_between_nodes
    }
}
