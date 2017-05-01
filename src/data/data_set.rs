use std::collections::HashMap;
use sprs::CsVecOwned;

pub trait DataSet {
    fn get_user_indices(&self) -> HashMap<String, usize>;
    fn get_item_indices(&self) -> HashMap<String, usize>;
    fn get_user_vectors(&self) -> HashMap<String, CsVecOwned<f64>>;
    fn get_item_vectors(&self) -> HashMap<String, CsVecOwned<f64>>;
}
