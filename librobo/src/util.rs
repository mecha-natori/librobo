#[cfg(any(feature = "alloc", feature = "heapless"))]
pub use vec::VecWrapper;

pub(crate) macro debug_log {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "log")]
        log::debug!(target: $target, $($arg)+);
    },
    ($($arg:tt)+) => {
        #[cfg(feature = "log")]
        log::debug!(target: "librobo", $($arg)+);
    }
}

pub(crate) macro trace_log {
    (target: $target:expr, $($arg:tt)+) => {
        #[cfg(feature = "log")]
        log::trace!(target: $target, $($arg)+);
    },
    ($($arg:tt)+) => {
        #[cfg(feature = "log")]
        log::trace!(target: "librobo", $($arg)+);
    }
}

#[cfg(any(feature = "alloc", feature = "heapless"))]
mod vec;
