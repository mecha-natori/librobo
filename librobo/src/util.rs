#[cfg(any(feature = "alloc", feature = "heapless"))]
pub use vec::VecWrapper;

#[cfg(any(feature = "alloc", feature = "heapless"))]
mod vec;
