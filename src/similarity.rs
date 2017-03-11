//! Module with some basic similarities, you can write your own using the sparse module

use std::collections::BTreeMap;
use super::sparse::*;

/// Computes the cosine similarity between two sparse vectors.
/// It returns 0.0 if one of the vectors has null norm.
#[allow(unused_variables)]
pub fn cosine(a: &BTreeMap<usize, f64>, b: &BTreeMap<usize, f64>, n: usize) -> f64 {
    let norms = norm(a)*norm(b);
    if norms > 0. {
        return dot(a,b)/(norm(a)*norm(b))
    }
    return 0.;
}

/// Computes the extended Jaccard similarity between two sparse vectors.
#[allow(unused_variables)]
pub fn jaccard(a: &BTreeMap<usize, f64>, b: &BTreeMap<usize, f64>, n: usize) -> f64 {
    let inner = dot(a,b);
    inner/(dot(a,a)+dot(b,b)-inner)
}

/// Computes the Pearson correlation index between two sparse vectors.
/// It treats the vectors as discrete uniform random variables.
pub fn pearson(a: &BTreeMap<usize, f64>, b: &BTreeMap<usize, f64>, n: usize) -> f64 {
    covariance(a,b,n)/(covariance(a,a,n).sqrt()*covariance(b,b,n).sqrt())
}
