/// generic N dimensional matrix that can contain any types implementing
/// the Copy and Default traits.
pub struct Matrix<'a, T: Copy, const N: usize> {
    dim: [usize; N],
    elems: &'a [T],
}

impl<'a, T: Copy, const N: usize> Matrix<'a, T, N> {
    /// construct a new matrix with the provided dimensionality.
    pub fn new(dim: [usize; N], elems: &'a [T]) -> Self {
        let size = dim.iter().fold(1, |acc, elem| acc * elem);
        if size != elems.len() {
            panic!(
                "unexpected size for elems, should be same as all dimensions multiplied together"
            );
        }
        Self { dim, elems }
    }

    fn index(&self, loc: [usize; N]) -> usize {
        let mut mul = 1;
        let mut index = 0;
        for i in 0..N {
            index += mul * loc[i];
            mul *= self.dim[i];
        }
        index
    }

    /// get the element at the provided location.
    /// you must ensure that the location is within the bounds,
    /// otherwise the function may return the wrong T, or panic.
    pub fn get(&self, loc: [usize; N]) -> T {
        self.elems[self.index(loc)]
    }

    /// returns the dimensions of the Matrix
    pub fn dim(&self) -> [usize; N] {
        self.dim
    }
}
