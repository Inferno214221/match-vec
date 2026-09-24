use std::mem::{self, MaybeUninit};

pub const fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    [const { MaybeUninit::uninit() }; N]
}

pub const fn take_uninit<T>(dest: &mut MaybeUninit<T>) -> MaybeUninit<T> {
    mem::replace(dest, MaybeUninit::uninit())
}

/// An extension trait that provides methods for moving a constant number of items out of an owned
/// slice. All methods are unsafe because they perform no checks to ensure that the correct number
/// of items are present, this is the responsibility of the caller.
pub trait SliceExt<T: Sized> {
    /// Removes the first N items from this slice, returning them as an array. Remaining items are
    /// preserved, with self updated accordingly.
    ///
    /// # Safety
    /// This slice must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_front<const N: usize>(&mut self) -> [T; N];

    /// Removes the last N items from this slice, returning them as an array. Remaining items are
    /// preserved, with self updated accordingly.
    ///
    /// # Safety
    /// This slice must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_back<const N: usize>(&mut self) -> [T; N];

    /// Removes the first F and last B items from this slice, returning them as two separate arrays.
    /// Remaining items are preserved, with self updated accordingly.
    ///
    /// # Safety
    /// This slice must contain at least F + B items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]);

    /// Removes all items from this slice, returning them as an array.
    ///
    /// # Safety
    /// This slice must contain exactly N items. Failure to ensure this is undefined behaviour.
    unsafe fn take_all_exact<const N: usize>(self) -> [T; N];

    /// Removes the first N items from this slice, returning them as an array. Remaining items and
    /// self are dropped in place.
    ///
    /// # Safety
    /// This slice must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn take_front<const N: usize>(self) -> [T; N];

    /// Removes the last N items from this slice, returning them as an array. Remaining items and
    /// self are dropped in place.
    ///
    /// # Safety
    /// This slice must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn take_back<const N: usize>(self) -> [T; N];

    /// Removes the first F and last B items from this slice, returning them as two separate arrays.
    /// Remaining items and self are dropped in place.
    ///
    /// # Safety
    /// This slice must contain at least F + B items. Failure to ensure this is undefined behaviour.
    unsafe fn take_both<const F: usize, const B: usize>(self) -> ([T; F], [T; B]);
}