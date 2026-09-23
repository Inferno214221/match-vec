use std::mem::{self, MaybeUninit};

const fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    [const { MaybeUninit::uninit() }; N]
}

const fn take_uninit<T>(dest: &mut MaybeUninit<T>) -> MaybeUninit<T> {
    mem::replace(dest, MaybeUninit::uninit())
}

pub trait VecExt<T: Sized> {
    /// Removes the first N items from this Vec, returning them as an array.
    ///
    /// # Safety
    /// The Vec must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_front_n<const N: usize>(&mut self) -> [T; N];

    /// Removes the last N items from this Vec, returning them as an array.
    ///
    /// # Safety
    /// The Vec must contain at least N items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_back_n<const N: usize>(&mut self) -> [T; N];

    /// Removes the first F and last B items from this Vec, returning them as two separate arrays.
    ///
    /// # Safety
    /// The Vec must contain at least F + B items. Failure to ensure this is undefined behaviour.
    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]);
}

impl<T: Sized> VecExt<T> for Vec<T> {
    unsafe fn pop_front_n<const N: usize>(&mut self) -> [T; N] {
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

    unsafe fn pop_back_n<const N: usize>(&mut self) -> [T; N] {
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
}