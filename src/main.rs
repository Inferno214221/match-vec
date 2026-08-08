use match_vec_macro::match_vec;

fn take_0() {
    println!("0 args")
}

fn take_1(a: usize) {
    println!("1 args: {a}")
}

fn take_vec(vec: Vec<usize>) {
    println!("vec: {vec:?}")
}

fn take_vec_and_1(vec: Vec<usize>, a: usize) {
    println!("vec and 1 args: {vec:?}, {a}")
}

fn take_2_and_vec(a: usize, b: usize, vec: Vec<usize>) {
    println!("2 args and vec: {a}, {b}, {vec:?}")
}

#[derive(Debug, PartialEq, Eq)]
pub struct NC(pub usize);

fn main() {
    let mut vec = vec![NC(0), NC(2), NC(3), NC(4)];

    mod mine {
        use super::NC;
        pub const ZERO: NC = NC(0);
    }

    match_vec!(match vec {
        [] => {
            println!("zero element slice match")
        },
        [a, b, c] => {
            println!("{a:?}, {b:?}, {c:?}")
        },
        [mine::ZERO, b, start @ ..] => {
            println!("0, {b:?}, {start:?}")
        },
        [start @ .., a] => {
            println!("{start:?}, {a:?}")
        },
        _ => {
            println!("other")
        },
    });

    // match &vec[..] {
    //     /* [] */ [] => {
    //         take_0()
    //     },
    //     /* [a] */ [_] => {
    //         unsafe { vec.set_len(0) };
    //         let spare = vec.spare_capacity_mut();
    //         let a = unsafe { spare[0].assume_init() };

    //         take_1(a)
    //     },
    //     /* [0, rest @ ..] */ [0, ..] => {
    //         let mut drain = vec.drain(..1);
    //         let _ = drain.next();
    //         ::std::mem::drop(drain);

    //         let rest = vec;
    //         take_vec(rest);
    //     }
    //     // /* [start @ .., a] */ [slice @ .., _] => {
    //     //     let rem_len = slice.len();

    //     //     unsafe { vec.set_len(rem_len - 1) };
    //     //     let spare = vec.spare_capacity_mut();
    //     //     let a = unsafe { spare[0].assume_init() };

    //     //     let start = vec;

    //     //     take_vec_and_1(start, a)
    //     // },
    //     /* [a, mid @ .., b] */ [_, slice @ .., _] => {
    //         let rem_len = slice.len();

    //         let mut drain = vec.drain(..1);
    //         let a = unsafe { drain.next().unwrap_unchecked() };
    //         ::std::mem::drop(drain);

    //         unsafe { vec.set_len(rem_len - 1) };
    //         let spare = vec.spare_capacity_mut();
    //         let b = unsafe { spare[0].assume_init() };

    //         let mid = vec;

    //         take_2_and_vec(a, b, mid)
    //     },
    //     // /* all */ _ => {
    //     //     let all = vec;
    //     //     take_vec(all)
    //     // }
    // }
}


// let mut vec = vec![0, 1, 2, 3];

// // no partial moves, they would result in a memory leak
// match vec.len() {
//     /* [a, b, rest @ ..] */ n if n > 2 => {
//         let mut drain = vec.drain(..2);
//         let a = unsafe { drain.next().unwrap_unchecked() };
//         let b = unsafe { drain.next().unwrap_unchecked() };
//         ::std::mem::drop(drain);
//         let rest = vec;
//         take_2_and_vec(a, b, rest);
//     },
//     /* [start @ .., a] */ n if n > 1 => {
//         unsafe { vec.set_len(n - 1) };
//         let spare = vec.spare_capacity_mut();
//         let a = unsafe { spare[0].assume_init() };
//         let start = vec;
//         take_vec_and_1(start, a)
//     },
//     /* [a] */ 1 => {
//         unsafe { vec.set_len(0) };
//         let spare = vec.spare_capacity_mut();
//         let a = unsafe { spare[0].assume_init() };
//         take_1(a)
//     },
//     /* [] */ 0 => {
//         take_0()
//     },
//     _ => {
//         //
//     }
// }