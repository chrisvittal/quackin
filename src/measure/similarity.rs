use sprs::CsVecOwned;

pub fn cosine(a: &CsVecOwned<f64>, b: &CsVecOwned<f64>) -> f64 {
    let norms = a.dot(a)*b.dot(b);
    if norms > 0.0 {
        a.dot(b) / norms.sqrt()
    }
    else {
        0.0
    }
}
