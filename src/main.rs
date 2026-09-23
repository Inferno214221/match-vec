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

// TODO: boxed slice too, but with no catchall - could easily have a move catchall

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonCopy(pub usize);

fn main() {
    let vec = vec![NonCopy(0), NonCopy(2), NonCopy(3), NonCopy(4)];

    mod mine {
        use super::NonCopy;
        pub const ZERO: NonCopy = NonCopy(0);
    }

    match_vec!(match vec {
        ref vec_ref => {
            take_vec(vec_ref.clone())
        },
        [a, _, ref c] => {
            take_2(a, c.clone())
        },
        [NonCopy(0), b, ref start @ .., _] => {
            take_vec_and_2(start.clone(), mine::ZERO, b)
        },
        [start @ .., a] => {
            take_vec_and_1(start, a)
        },
    });
}