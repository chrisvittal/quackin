//! Module with some basic similarities between vectors

use std::collections::HashMap;
use super::sparse::*;

/// Computes the cosine similarity between two sparse vectors.
/// It returns 0.0 if one of the vectors has null norm.
#[allow(unused_variables)]
pub fn cosine(a: &HashMap<usize, f64>, b: &HashMap<usize, f64>, n: usize) -> f64 {
    let norms = norm(a)*norm(b);
    if norms > 0. {
        return dot(a,b)/(norm(a)*norm(b))
    }
    return 0.;
}

/// Computes the extended Jaccard similarity between two sparse vectors.
#[allow(unused_variables)]
pub fn jaccard(a: &HashMap<usize, f64>, b: &HashMap<usize, f64>, n: usize) -> f64 {
    let inner = dot(a,b);
    inner/(dot(a,a)+dot(b,b)-inner)
}

/// Computes the Pearson correlation index between two sparse vectors.
/// It treats the vectors as discrete uniform random variables.
pub fn pearson(a: &HashMap<usize, f64>, b: &HashMap<usize, f64>, n: usize) -> f64 {
    covariance(a,b,n)/(covariance(a,a,n).sqrt()*covariance(b,b,n).sqrt())
}
