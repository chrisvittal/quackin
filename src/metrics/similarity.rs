//! This submodule provides some basic similarity measures
//!
//! It supports sparse vectors from `sprs` which seems to be the most popular
//! library for sparse algebra.

use sprs::CsVecOwned;

/// Type for a similarity function
pub type Similarity = fn(&CsVecOwned<f64>, &CsVecOwned<f64>) -> f64;

/// Cosine similarity between two vectors.
///
/// Returns zero if one of the vectors is zero.
pub fn cosine(a: &CsVecOwned<f64>, b: &CsVecOwned<f64>) -> f64 {
    let norms = a.dot(a) * b.dot(b);
    if norms > 0.0 {
        a.dot(b)/norms.sqrt()
    } else {
        0.0
    }
}
