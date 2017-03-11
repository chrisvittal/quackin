//! Module with some functions that implement basic algebraic operations between sparse vectors.
//! Here a sparse vector is just a `HashMap<ID, f64>`

use std::collections::HashMap;

/// Computes the dot product between two sparse vectors.
pub fn dot(a: &HashMap<usize, f64>, b: &HashMap<usize, f64>) -> f64 {
    a.iter().map(|(key, val_a)| {
        match b.get(key) {
            Some(val_b) => val_a*val_b,
            None => 0.
        }
    }).sum()
}

/// Computes the norm of an sparse vector.
pub fn norm(a: &HashMap<usize, f64>) -> f64 {
    dot(a,a).sqrt()
}

/// Computes the covariance between two sparse vectors.
/// It treats the vectors as discrete uniform random variables.
pub fn covariance(a: &HashMap<usize, f64>, b: &HashMap<usize, f64>, n: usize) -> f64 {
    let sum_a: f64 = a.values().sum();
    let sum_b: f64 = b.values().sum();
    dot(a,b) - (sum_a*sum_a + sum_b*sum_b)/(n as f64)
}
