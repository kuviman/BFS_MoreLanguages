use super::{point, Point};

pub struct Array2D<T> {
    inner: [T; point::MAX_COORD * point::MAX_COORD],
}

impl<T> Array2D<T> {
    fn index(&self, pos: Point) -> usize {
        pos.index() as usize
    }
}

impl<T: Default + Copy> Array2D<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        Self::filled_with(T::default(), width, height)
    }
}

impl<T: Copy> Array2D<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Self {
        let mut result = unsafe { Self::uninitialized(width, height) };
        result.fill(value);
        result
    }
    pub unsafe fn uninitialized(width: usize, height: usize) -> Self {
        assert!(width <= point::MAX_COORD);
        assert!(height <= point::MAX_COORD);
        Self {
            inner: std::mem::MaybeUninit::uninit().assume_init(),
        }
    }
    pub fn fill(&mut self, value: T) {
        for x in &mut self.inner {
            *x = value;
        }
    }
}

impl<T> std::ops::Index<Point> for Array2D<T> {
    type Output = T;
    fn index(&self, pos: Point) -> &T {
        let index: usize = self.index(pos);
        #[cfg(not(feature = "unsafe-indexing"))]
        return &self.inner[index];
        #[cfg(feature = "unsafe-indexing")]
        return unsafe { self.inner.get_unchecked(index) };
    }
}

impl<T> std::ops::IndexMut<Point> for Array2D<T> {
    fn index_mut(&mut self, pos: Point) -> &mut T {
        let index: usize = self.index(pos);
        #[cfg(not(feature = "unsafe-indexing"))]
        return &mut self.inner[index];
        #[cfg(feature = "unsafe-indexing")]
        return unsafe { self.inner.get_unchecked_mut(index) };
    }
}
