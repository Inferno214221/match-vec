use std::mem::{self, MaybeUninit};

use crate::util::UninitArray;

use super::{SliceExt, take_uninit};

fn boxed_slice_to_uninit<T: Sized>(this: Box<[T]>) -> Box<[MaybeUninit<T>]> {
    let ptr = Box::into_raw(this);
    let ptr = ptr as *mut [MaybeUninit<T>];

    // SAFETY: The slice has exactly the same layout as before, but items are now MaybeUninit.
    unsafe { Box::from_raw(ptr) }
}

impl<T: Sized> SliceExt<T> for Box<[T]> {
    unsafe fn pop_front<const N: usize>(&mut self) -> [T; N] {
        let mut boxed = boxed_slice_to_uninit(mem::take(self));

        let len = boxed.len();
        let slice = &mut *boxed;

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut slice[i]);
        }

        let mut new = Box::new_uninit_slice(len - N);
        for i in N..len {
            new[i - N] = take_uninit(&mut slice[i]);
        }

        drop(boxed);

        unsafe {
            *self = new.assume_init();
            popped.assume_init()
        }
    }

    unsafe fn pop_back<const N: usize>(&mut self) -> [T; N] {
        let mut boxed = boxed_slice_to_uninit(mem::take(self));

        let len = boxed.len();
        let rem_end = len - N;
        let slice = &mut *boxed;

        let mut new = Box::new_uninit_slice(rem_end);
        for i in 0..rem_end {
            new[i] = take_uninit(&mut slice[i]);
        }

        let mut popped = UninitArray::<T, N>::new();
        for i in rem_end..len {
            popped[i - rem_end] = take_uninit(&mut slice[i]);
        }

        drop(boxed);

        unsafe {
            *self = new.assume_init();
            popped.assume_init()
        }
    }

    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]) {
        let mut boxed = boxed_slice_to_uninit(mem::take(self));

        let len = boxed.len();
        let rem_end = len - B;
        let slice = &mut *boxed;

        let mut popped_front = UninitArray::<T, F>::new();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut slice[i]);
        }

        let mut new = Box::new_uninit_slice(rem_end - F);
        for i in F..rem_end {
            new[i - F] = take_uninit(&mut slice[i]);
        }

        let mut popped_back = UninitArray::<T, B>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut slice[i]);
        }

        drop(boxed);

        unsafe {
            *self = new.assume_init();
            (
                popped_front.assume_init(),
                popped_back.assume_init()
            )
        }
    }

    unsafe fn take_all_exact<const N: usize>(self) -> [T; N] {
        let ptr = Box::into_raw(self);
        let ptr = ptr as *mut [T; N];

        // SAFETY: The underlying array of a slice has the exact same layout as an actual array
        // `[T; N]` if `N` is equal to the slice's length.
        let boxed = unsafe { Box::from_raw(ptr) };
        *boxed
    }

    unsafe fn take_front<const N: usize>(mut self) -> [T; N] {
        let mut boxed = boxed_slice_to_uninit(mem::take(&mut self));

        let slice = &mut *boxed;

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut slice[i]);
        }

        for item in &mut slice[N..] {
            unsafe { item.assume_init_drop() };
        }

        drop(boxed);

        unsafe {
            popped.assume_init()
        }
    }

    unsafe fn take_back<const N: usize>(mut self) -> [T; N] {
        let mut boxed = boxed_slice_to_uninit(mem::take(&mut self));

        let len = boxed.len();
        let rem_end = len - N;
        let slice = &mut *boxed;

        for item in &mut slice[..rem_end] {
            unsafe { item.assume_init_drop() };
        }

        let mut popped_back = UninitArray::<T, N>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut slice[i]);
        }

        drop(boxed);

        unsafe {
            popped_back.assume_init()
        }
    }

    unsafe fn take_both<const F: usize, const B: usize>(mut self) -> ([T; F], [T; B]) {
        let mut boxed = boxed_slice_to_uninit(mem::take(&mut self));

        let len = boxed.len();
        let rem_end = len - B;
        let slice = &mut *boxed;

        let mut popped_front = UninitArray::<T, F>::new();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut slice[i]);
        }

        for item in &mut slice[F..rem_end] {
            unsafe { item.assume_init_drop() };
        }

        let mut popped_back = UninitArray::<T, B>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut slice[i]);
        }

        drop(boxed);

        unsafe {
            (
                popped_front.assume_init(),
                popped_back.assume_init()
            )
        }
    }
}