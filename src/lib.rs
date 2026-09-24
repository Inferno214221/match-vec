extern crate self as match_vec;

pub use match_vec_macro::match_vec;

#[doc(hidden)]
pub mod internal {
    pub use match_vec_internal::SliceExt;
}