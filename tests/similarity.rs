extern crate sprs;
extern crate oozie;

use sprs::{CsVecOwned};
use oozie::measure::similarity::*;

#[test]
fn cosine_zero_test() {
    let a = CsVecOwned::new(3, vec![0, 1, 2], vec![1.0, 1.0, 1.0]);
    let b = CsVecOwned::new(3, vec![], vec![]);
    assert_eq!(0.0, cosine(&a, &b));
}

#[test]
fn cosine_parallel_test() {
    let a = CsVecOwned::new(3, vec![0, 1, 2], vec![1.0, 1.0, 1.0]);
    let b = CsVecOwned::new(3, vec![0, 1, 2], vec![2.0, 2.0, 2.0]);
    assert_eq!(1.0, cosine(&a, &b));
}

#[test]
fn cosine_orthogonal_test() {
    let a = CsVecOwned::new(3, vec![0, 1, 2], vec![1.0, 1.0, 0.0]);
    let b = CsVecOwned::new(3, vec![0, 1, 2], vec![2.0, -2.0, 0.0]);
    assert_eq!(0.0, cosine(&a, &b));
}
