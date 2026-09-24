#![feature(iterator_try_collect)]
#![feature(maybe_uninit_array_assume_init)]

pub mod args;
pub mod entry;
pub mod slice_ext;
pub mod generate_body;
pub mod to_tokens;

pub use args::*;
pub use entry::*;
pub use slice_ext::*;
pub use generate_body::*;