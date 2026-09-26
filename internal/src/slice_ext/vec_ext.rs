use crate::util::UninitArray;

use super::{SliceExt, take_uninit};

impl<T: Sized> SliceExt<T> for Vec<T> {
    unsafe fn pop_front<const N: usize>(&mut self) -> [T; N] {
        let len = self.len();
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for i in N..len {
            spare[i - N] = take_uninit(&mut spare[i]);
        }

        unsafe {
            self.set_len(len - N);
            popped.assume_init()
        }
    }

    unsafe fn pop_back<const N: usize>(&mut self) -> [T; N] {
        let len = self.len();
        unsafe { self.set_len(len - N) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        unsafe { popped.assume_init() }
    }

    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]) {
        let len = self.len();
        let rem_end = len - B;
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped_front = UninitArray::<T, F>::new();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut spare[i]);
        }

        for i in F..rem_end {
            spare[i - F] = take_uninit(&mut spare[i]);
        }

        let mut popped_back = UninitArray::<T, B>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut spare[i]);
        }

        unsafe {
            self.set_len(rem_end - F);
            (
                popped_front.assume_init(),
                popped_back.assume_init()
            )
        }
    }

    unsafe fn take_all_exact<const N: usize>(mut self) -> [T; N] {
        unsafe { self.pop_back() }
    }

    unsafe fn take_front<const N: usize>(mut self) -> [T; N] {
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[N..] {
            unsafe { item.assume_init_drop() };
        }

        unsafe {
            popped.assume_init()
        }
    }

    unsafe fn take_back<const N: usize>(mut self) -> [T; N] {
        unsafe { self.pop_back() }
    }

    unsafe fn take_both<const F: usize, const B: usize>(mut self) -> ([T; F], [T; B]) {
        let len = self.len();
        let rem_end = len - B;
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped_front = UninitArray::<T, F>::new();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[F..rem_end] {
            unsafe { item.assume_init_drop() };
        }

        let mut popped_back = UninitArray::<T, B>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut spare[i]);
        }

        unsafe {
            (
                popped_front.assume_init(),
                popped_back.assume_init()
            )
        }
    }
}