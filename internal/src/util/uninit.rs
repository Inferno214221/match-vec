use std::{mem::{ManuallyDrop, MaybeUninit}, ops::{Deref, DerefMut}};

/// A helper type for working with `[MaybeUninit<T>; N]`. Also unfortunately the only stable way to
/// convert between `[MaybeUninit<T>; N]` and `[T; N]`. This union acts is if it always contains
/// uninitialized contents until explicitly assumed otherwise and moved out using
/// [`UninitArray::assume_init`]. As such, it implements [`Deref`] and [`DerefMut`] with
/// `Target = [MaybeUninit<T>; N]`.
///
/// As with `[MaybeUninit<T>; N]`, this type does not call drop on any contained values unless
/// explicitly instructed with [`UninitArray::assume_init_drop`].
pub union UninitArray<T, const N: usize> {
    uninit: ManuallyDrop<[MaybeUninit<T>; N]>,
    init: ManuallyDrop<[T; N]>,
}

impl<T, const N: usize> UninitArray<T, N> {
    /// Creates a new `UninitArray` that contains N uninitialized values.
    pub const fn new() -> UninitArray<T, N> {
        UninitArray {
            uninit: ManuallyDrop::new([const { MaybeUninit::uninit() }; N])
        }
    }

    /// Assumes that the contents of this array are initialized and deconstructs it.
    ///
    /// # Safety
    /// It is the callers responsibility to ensure that the contents of the array are
    /// actually initialized. Failure to do so is undefined behaviour.
    pub const unsafe fn assume_init(self) -> [T; N] {
        // SAFETY:
        // - `ManuallyDrop` and `MaybeUninit` are both repr(transparent), so the `uninit` and
        //   `init` fields have the same size, alignment.
        // - The user guarantees that all values contained in this union are now initialized.
        unsafe { ManuallyDrop::into_inner(self.init) }
    }

    /// Assumes that the contents of this array are initialized and drops it in place.
    ///
    /// # Safety
    /// It is the callers responsibility to ensure that the contents of the array are
    /// actually initialized. Failure to do so is undefined behaviour.
    pub unsafe fn assume_init_drop(mut self) {
        // SAFETY:
        // - `ManuallyDrop` and `MaybeUninit` are both repr(transparent), so the `uninit` and
        //   `init` fields have the same size, alignment.
        // - The user guarantees that all values contained in this union are now initialized.
        unsafe { ManuallyDrop::drop(&mut self.init) }
    }
}

impl<T, const N: usize> Default for UninitArray<T, N> {
    fn default() -> Self {
        UninitArray::new()
    }
}

impl<T, const N: usize> Deref for UninitArray<T, N> {
    type Target = [MaybeUninit<T>; N];

    fn deref(&self) -> &Self::Target {
        // SAFETY: Unless deconstructed, `UninitArray` is treated as a `[MaybeUninit<T>; N]`. Even if
        // initialized, all values of `[T; N]` are valid as `[MaybeUninit<T>; N]`.
        unsafe { &self.uninit }
    }
}

impl<T, const N: usize> DerefMut for UninitArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Unless deconstructed, `UninitArray` is treated as a `[MaybeUninit<T>; N]`. Even if
        // initialized, all values of `[T; N]` are valid as `[MaybeUninit<T>; N]`.
        unsafe { &mut self.uninit }
    }
}