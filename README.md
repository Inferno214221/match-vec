# Match Vec (`match_vec`)

A proc macro for matching and moving out of a `Vec`.

## Motivation

The other day I made the mistake of trying to write the following code:

```rust
let anchors: Vec<TokenStream>;

/* Some code that initializes anchors with somewhere between 0 and 3 values. */

match anchors {
    [] => quote!(::ct_regex::internal::anchor::AnchorNone),
    [a] => a,
    [a, b] => quote!(::ct_regex::internal::anchor::AnchorPair<#a, #b>),
    [a, b, c] => quote!(::ct_regex::internal::anchor::AnchorSet<#a, #b, #c>),
    _ => panic!("an excessive number for anchor assertions were found"),
}
```

When compiling, I was met with the following error:

```
error[E0529]: expected an array or slice, found `Vec<TokenStream>`
  --> internal/src/codegen/create_type/parse.rs:61:13
   |
61 |             [] => quote!(::ct_regex::internal::anchor::AnchorNone),
   |             ^^ pattern cannot match with input type `Vec<TokenStream>`
   |
help: consider slicing here
   |
60 |         match anchors[..] {
   |                      ++++
```

I then adding `[..]` as recommended, and was met with:

```
error[E0508]: cannot move out of type `[TokenStream]`, a non-copy slice
  --> internal/src/codegen/create_type/parse.rs:60:15
   |
60 |         match anchors[..] {
   |               ^^^^^^^^^^^ cannot move out of here
61 |             [] => quote!(::ct_regex::internal::anchor::AnchorNone),
62 |             [a] => a,
   |              - data moved here
63 |             [a, b] => quote!(::ct_regex::internal::anchor::AnchorPair<#a, #b>),
   |              -  - ...and here
   |              |
   |              ...and here
64 |             [a, b, c] => quote!(::ct_regex::internal::anchor::AnchorSet<#a, #b, #c>),
   |              -  -  - ...and here
   |              |  |
   |              |  ...and here
   |              ...and here
   |
   = note: move occurs because these variables have types that don't implement the `Copy` trait
help: consider borrowing here
   |
60 |         match &anchors[..] {
   |               +
```

For the use case I had, borrowing the elements of `anchors` was not a problem, as I could just call
`clone` in the one branch where I actually needed an owned value. The contents where only a short
`proc_macro2::TokenTree`, running in ... _real shocker here:_ a different macro. The resulting code
was:

```rust
/* snip */

match &anchors[..] {
    [] => quote!(::ct_regex::internal::anchor::AnchorNone),
    [a] => a.clone(),
    // Actually, I changed the line above to quote!(#a) for consistency, but it has the same effect.
    [a, b] => quote!(::ct_regex::internal::anchor::AnchorPair<#a, #b>),
    [a, b, c] => quote!(::ct_regex::internal::anchor::AnchorSet<#a, #b, #c>),
    _ => panic!("an excessive number for anchor assertions were found"),
}
```

But for a language that usually provides a significant amount of control over when values are moved,
I was left wondering why there wasn't an easy way to match against an owned `Vec`, moving the values
in the process.

special casing a type that is literally defined as a struct...

Obviously, the main difficulty here is that non-empty `Vec`s own a heap allocation which needs to be
deallocated after the values are moved out. But there exist many ways to move values out of a `Vec`s
heap: `pop`, `drain`, `remove`, etc. Moving off of the heap it understood by the compiler, even if
[only when dereferencing a
`Box`](https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special/).

## Functionality

Anyway, it's not all that hard to write code that matches against the sliced `Vec`, moves those
values off of the heap and binds them to variables. That's what this macro does, resulting in an
expression very similar to matching on a reference to a slice. It lets you write code like the
following:

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct NonCopy(pub usize);

fn main() {
    let mut vec = vec![NonCopy(0), NonCopy(2), NonCopy(3), NonCopy(4)];
    
    match_vec!(match vec {
        [] => {
            take_0()
        },
        [a, _, c] => {
            take_2(a, c)
        },
        [NonCopy(0), b, start @ ..] => {
            take_vec_and_2(start, NonCopy(0), b)
        },
        [start @ .., a] => {
            take_vec_and_1(start, a)
        },
    });
}
```

**EXAMPLE UPDATE PENDING**