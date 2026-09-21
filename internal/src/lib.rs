#![feature(iterator_try_collect)]
#![feature(maybe_uninit_array_assume_init)]

pub mod args;
pub mod entry;
pub mod generate_body;
pub mod to_tokens;
pub mod vec_ext;

pub use args::*;
pub use entry::*;
pub use generate_body::*;
pub use vec_ext::*;