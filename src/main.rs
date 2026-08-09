use match_vec_macro::match_vec;

fn take_0() {
    println!("0 args")
}

fn take_1(a: NonCopy) {
    println!("1 args: {a:?}")
}

fn take_2(a: NonCopy, b: NonCopy) {
    println!("2 args: {a:?}, {b:?}")
}

fn take_vec(vec: Vec<NonCopy>) {
    println!("vec: {vec:?}")
}

fn take_vec_and_1(vec: Vec<NonCopy>, a: NonCopy) {
    println!("vec and 1 args: {vec:?}, {a:?}")
}

fn take_vec_and_2(vec: Vec<NonCopy>, a: NonCopy, b: NonCopy) {
    println!("vec and 2 args: {vec:?}, {a:?}, {b:?}")
}

#[derive(Debug, PartialEq, Eq)]
pub struct NonCopy(pub usize);

fn main() {
    let mut vec = vec![NonCopy(0), NonCopy(2), NonCopy(3), NonCopy(4)];

    mod mine {
        use super::NonCopy;
        pub const ZERO: NonCopy = NonCopy(0);
    }

    match_vec!(match vec {
        [] => {
            take_0()
        },
        [a, _, c] => {
            take_2(a, c)
        },
        [NonCopy(0), b, start @ ..] => {
            take_vec_and_2(start, mine::ZERO, b)
        },
        [start @ .., a] => {
            take_vec_and_1(start, a)
        },
    });
}