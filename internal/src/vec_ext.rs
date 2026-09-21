use std::mem::{self, MaybeUninit};

pub trait VecExt<T: Sized> {
    unsafe fn pop_front_n<const N: usize>(&mut self) -> [T; N];
    unsafe fn pop_back_n<const N: usize>(&mut self) -> [T; N];
    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]);
}

impl<T: Sized> VecExt<T> for Vec<T> {
    unsafe fn pop_front_n<const N: usize>(&mut self) -> [T; N] {
        let mut popped: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let mut drain = self.drain(0..N);
        for i in &mut popped {
            *i = MaybeUninit::new(unsafe { drain.next().unwrap_unchecked() });
        }
        drop(drain);
        unsafe { MaybeUninit::array_assume_init(popped) }
    }

    unsafe fn pop_back_n<const N: usize>(&mut self) -> [T; N] {
        let mut popped: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let len = self.len();
        unsafe { self.set_len(len - N) };
        let spare = self.spare_capacity_mut();
        for i in 0..N {
            popped[i] = MaybeUninit::new(unsafe { spare[i].assume_init_read() });
        }
        unsafe { MaybeUninit::array_assume_init(popped) }
    }

    unsafe fn pop_both<const F: usize, const B: usize>(&mut self) -> ([T; F], [T; B]) {
        unsafe { (self.pop_front_n::<F>(), self.pop_back_n::<B>()) }
    }
}