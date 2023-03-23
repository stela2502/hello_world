#![allow(
    // clippy is broken and shows wrong warnings
    // clippy on stable does not know yet about the lint name
    unknown_lints,
    // https://github.com/rust-lang/rust-clippy/issues/8560
    clippy::only_used_in_recursion,
    // https://github.com/rust-lang/rust-clippy/issues/8867
    clippy::derive_partial_eq_without_eq,
    // https://github.com/rust-lang/rust-clippy/issues/9101
    clippy::explicit_auto_deref
)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub mod geneids;
pub mod ifiles;
pub mod ofiles;