#[cfg(feature = "alloc")]
use alloc::vec::Vec as AVec;
use core::ops::Index;
use core::ops::IndexMut;
#[cfg(feature = "heapless")]
use heapless::Vec as HVec;

pub struct VecWrapper<'a, T, const N: usize> {
    #[cfg(feature = "alloc")]
    vec_a: &'a mut AVec<T>,
    #[cfg(feature = "heapless")]
    vec_h: &'a mut HVec<T, N>
}

#[cfg(feature = "alloc")]
impl<'a, T, const N: usize> From<&'a mut AVec<T>> for VecWrapper<'a, T, N> {
    fn from(vec_a: &'a mut AVec<T>) -> Self {
        Self { vec_a }
    }
}

#[cfg(feature = "heapless")]
impl<'a, T, const N: usize> From<&'a mut HVec<T, N>> for VecWrapper<'a, T, N> {
    fn from(vec_h: &'a mut HVec<T, N>) -> Self {
        Self { vec_h }
    }
}

impl<T, const N: usize> Index<usize> for VecWrapper<'_, T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        #[cfg(feature = "alloc")]
        return &self.vec_a[index];
        #[cfg(feature = "heapless")]
        return &self.vec_h[index];
    }
}

impl<T, const N: usize> IndexMut<usize> for VecWrapper<'_, T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[cfg(feature = "alloc")]
        return &mut self.vec_a[index];
        #[cfg(feature = "heapless")]
        return &mut self.vec_h[index];
    }
}
