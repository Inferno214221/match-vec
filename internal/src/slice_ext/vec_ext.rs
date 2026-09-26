use crate::util::UninitArray;

use super::{SliceExt, take_uninit};

impl<T: Sized> SliceExt<T> for Vec<T> {
    unsafe fn pop_front<const N: usize>(&mut self) -> [T; N] {
        let len = self.len();

        // SAFETY: We set the length of the `Vec` to 0, but `len` items remain initialized.
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for i in N..len {
            spare[i - N] = take_uninit(&mut spare[i]);
        }

        // SAFETY: Documented for each expression.
        unsafe {
            // SAFETY: All items `N..len` have been shifted move back `N` places, so the original
            // `Vec` is initialized for `len - N` items.
            self.set_len(len - N);
            // SAFETY: The first `N` items were written to `popped` before being overwritten, and
            // therefore remain initialized without being duplicated.
            popped.assume_init()
        }
    }

    unsafe fn pop_back<const N: usize>(&mut self) -> [T; N] {
        let len = self.len();

        // SAFETY: We reduce the length by `N`, so all values `(len - N)..len` remain
        // initialized.
        unsafe { self.set_len(len - N) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        // SAFETY: `popped` contains the last `N` items, all of which are initialized but forgotten
        // by the original `Vec`.
        unsafe { popped.assume_init() }
    }

    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]) {
        let len = self.len();
        let rem_end = len - B;

        // SAFETY: We set the length of the `Vec` to 0, but `len` items remain initialized.
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

        // SAFETY: Documented for each expression.
        unsafe {
            // SAFETY: All items `F..(len - B)` have been shifted move back `F` places, so the
            // original `Vec` is initialized for `len - B - F` items.
            self.set_len(rem_end - F);
            (
                // SAFETY: The first `F` items were written to `popped_front` before being
                // overwritten, and therefore remain initialized without being duplicated.
                popped_front.assume_init(),
                // SAFETY: The last `B` items have been written to `popped_back` and forgotten in
                // the original `Vec`. They remain initialized.
                popped_back.assume_init()
            )
        }
    }

    unsafe fn take_all_exact<const N: usize>(mut self) -> [T; N] {
        // SAFETY: The caller guarantees that the `Vec` contains exactly `N` items, so we redirect
        // to `pop_back`, with the remaining empty `Vec` being implicitly dropped.
        unsafe { self.pop_back() }
    }

    unsafe fn take_front<const N: usize>(mut self) -> [T; N] {
        let len = self.len();

        // SAFETY: We set the length of the `Vec` to 0, but `len` items remain initialized.
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped = UninitArray::<T, N>::new();
        for i in 0..N {
            popped[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[N..len] {
            // SAFETY: Items `N..len` remain initialized without use, so we drop them in place.
            unsafe { item.assume_init_drop() };
        }

        // SAFETY: The first `N` items have been read once and written to `popped`. They remain
        // initialized, but have been forgotten by the original `Vec`.
        unsafe { popped.assume_init() }
    }

    unsafe fn take_back<const N: usize>(mut self) -> [T; N] {
        // SAFETY: The caller guarantees that the `Vec` at least `N` items, so we redirect to
        // `pop_back`, with the remaining empty `Vec` being implicitly dropped.
        unsafe { self.pop_back() }
    }

    unsafe fn take_both<const F: usize, const B: usize>(mut self) -> ([T; F], [T; B]) {
        let len = self.len();
        let rem_end = len - B;

        // SAFETY: We set the length of the `Vec` to 0, but `len` items remain initialized.
        unsafe { self.set_len(0) };
        let spare = self.spare_capacity_mut();

        let mut popped_front = UninitArray::<T, F>::new();
        for i in 0..F {
            popped_front[i] = take_uninit(&mut spare[i]);
        }

        for item in &mut spare[F..rem_end] {
            // SAFETY: Items `F..(len - B)` remain initialized without use, so we drop them in
            // place.
            unsafe { item.assume_init_drop() };
        }

        let mut popped_back = UninitArray::<T, B>::new();
        for i in rem_end..len {
            popped_back[i - rem_end] = take_uninit(&mut spare[i]);
        }

        // SAFETY: Documented for each expression.
        unsafe {
            (
                // SAFETY: The first `N` items were written to `popped` before being overwritten,
                // and therefore remain initialized without being duplicated.
                popped_front.assume_init(),
                // SAFETY: The last `B` items have been read once and written to `popped_back`. They
                // remain initialized, but have been forgotten by the original `Vec`.
                popped_back.assume_init()
            )
        }
    }
}