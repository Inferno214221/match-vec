use match_vec::match_vec;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonCopy(pub usize);

fn main() {
    let me_vec = vec![NonCopy(0), NonCopy(2), NonCopy(3), NonCopy(4)];

    mod mine {
        use super::NonCopy;
        pub const ZERO: NonCopy = NonCopy(0);
    }

    match_vec!(match me_vec {
        [mine::ZERO, a, .., ref b, mine::ZERO] => {
            take_2(a, b.clone())
        },
        [NonCopy(0), a, mut end @ ..] => {
            end.push(NonCopy(7));
            take_vec_and_2(end, mine::ZERO, a)
        },
        [a, _] => {
            take_1(a)
        },
        [ref start @ .., mut a] => {
            a.0 += 1;
            take_vec_and_1(start.to_owned(), a)
        },
        [] => {
            take_0()
        },
    });
}