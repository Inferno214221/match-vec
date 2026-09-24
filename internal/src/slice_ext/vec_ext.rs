use std::mem::{MaybeUninit};

use super::{SliceExt, take_uninit, uninit_array};

impl<T: Sized> SliceExt<T> for Vec<T> {
    unsafe fn pop_front<const N: usize>(&mut self) -> [T; N] {
        let rem_end = self.len();
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = uninit_array::<T, N>();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for i in N..rem_end {
            spare[i - N] = take_uninit(&mut spare[i]);
        }

        unsafe {
            self.set_len(rem_end - N);
            MaybeUninit::array_assume_init(popped)
        }
    }

    unsafe fn pop_back<const N: usize>(&mut self) -> [T; N] {
        let len = self.len();
        unsafe { self.set_len(len - N) };
        let spare = self.spare_capacity_mut();

        let mut popped = uninit_array::<T, N>();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        unsafe { MaybeUninit::array_assume_init(popped) }
    }

    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]) {
        let whole_len = self.len();
        let rem_end = whole_len - B;
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped_front = uninit_array::<T, F>();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut spare[i]);
        }

        for i in F..rem_end {
            spare[i - F] = take_uninit(&mut spare[i]);
        }

        let mut popped_back = uninit_array::<T, B>();
        for i in rem_end..whole_len {
            popped_back[i - rem_end] = take_uninit(&mut spare[i]);
        }

        unsafe {
            self.set_len(rem_end - F);
            (
                MaybeUninit::array_assume_init(popped_front),
                MaybeUninit::array_assume_init(popped_back)
            )
        }
    }

    unsafe fn take_all_exact<const N: usize>(mut self) -> [T; N] {
        unsafe { self.pop_back() }
    }

    unsafe fn take_front<const N: usize>(mut self) -> [T; N] {
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = uninit_array::<T, N>();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[N..] {
            unsafe { item.assume_init_drop() };
        }

        unsafe {
            MaybeUninit::array_assume_init(popped)
        }
    }

    unsafe fn take_back<const N: usize>(mut self) -> [T; N] {
        unsafe { self.pop_back() }
    }

    unsafe fn take_both<const F: usize, const B: usize>(mut self) -> ([T; F], [T; B]) {
        let whole_len = self.len();
        let rem_end = whole_len - B;
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped_front = uninit_array::<T, F>();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[F..rem_end] {
            unsafe { item.assume_init_drop() };
        }

        let mut popped_back = uninit_array::<T, B>();
        for i in rem_end..whole_len {
            popped_back[i - rem_end] = take_uninit(&mut spare[i]);
        }

        unsafe {
            (
                MaybeUninit::array_assume_init(popped_front),
                MaybeUninit::array_assume_init(popped_back)
            )
        }
    }
}