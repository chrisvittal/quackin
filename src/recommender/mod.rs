//! This submodule provides some recommenders and tools to implement new ones.
//!
//! This is focused on collaborative filtering. It is planned to implement the
//! following models:
//!
//! - User based nearest neighbors
//! - Item based nearest neighbors
//! - SVD
//! - SVD++
//! - TimeSVD++

use std::collections::{HashMap, BTreeMap};

use sprs::CsVecOwned;

use super::data::Record;
use super::metrics::similarity::Similarity;

/// K-nearest neigbors user based recommender.
#[allow(dead_code)]
pub struct KnnUserRecommender {
    user_indices: HashMap<String, usize>,
    item_indices: HashMap<String, usize>,
    user_ids: Vec<String>,
    item_ids: Vec<String>,
    ratings: HashMap<usize, CsVecOwned<f64>>,
    similarity: Similarity,
    n_neighbors: usize
}

impl KnnUserRecommender {
    /// Constructs a new recommender from a slice of records.
    pub fn from_records(records: &[Record], similarity: Similarity, n_neighbors: usize) -> Self {
        let mut user_indices = HashMap::<String, usize>::new();
        let mut item_indices = HashMap::<String, usize>::new();
        let mut user_ids = Vec::new();
        let mut item_ids = Vec::new();
        let mut pre_vectors = HashMap::<usize, (BTreeMap<usize, f64>)>::new();
        let (mut i, mut j) = (0, 0);

        for record in records {
            let user_id: String = record.0.clone();
            let item_id: String = record.1.clone();
            let rating: f64 = record.2;

            if !(user_ids.contains(&user_id)) {
                user_ids.push(user_id.clone());
                user_indices.insert(user_id.clone(), i);
                i += 1;
            }

            if !(item_ids.contains(&item_id)) {
                item_ids.push(item_id.clone());
                item_indices.insert(item_id.clone(), j);
                j += 1;
            }

            let user_index = user_indices[&user_id];
            let item_index = item_indices[&item_id];

            if pre_vectors.contains_key(&user_index) {
                let pre_vector = pre_vectors.get_mut(&user_index).unwrap();
                pre_vector.insert(item_index, rating);
            }
            else {
                let mut pre_vector = BTreeMap::new();
                pre_vector.insert(item_index, rating);
                pre_vectors.insert(user_index, pre_vector);
            }
        }

        let ratings = pre_vectors.into_iter()
            .map(|(k, ind_dat)| {
                let (ind, dat):(Vec<usize>, Vec<f64>) = ind_dat.into_iter().unzip();
                (k, CsVecOwned::new(item_indices.len(), ind, dat))
            })
            .collect();
        Self {
            user_indices: user_indices,
            item_indices: item_indices,
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings,
            similarity: similarity,
            n_neighbors: n_neighbors
        }
    }
    /// Predicts the rating for an item given by an user.
    ///
    /// Returns error if the user or item ids could not be found
    /// or if there is no users with a positive similarity.
    pub fn predict(&self, user_id: &str, item_id: &str) -> Result<f64, String> {
        let user_index = try!(self.user_indices.get(user_id).ok_or("User not found"));
        let item_index = try!(self.item_indices.get(item_id).ok_or("Item not found"));

        let vector = self.ratings.get(user_index).unwrap();

        let mut ratings = self.ratings.iter().filter_map(|(other_index, other)| {
            if let Some(&rating) = other.get(*item_index) {
                if other_index != user_index {
                    Some((rating, (self.similarity)(vector, other)))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect::<Vec<_>>();

        ratings.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
        ratings.truncate(self.n_neighbors);

        let (rating, norm) = ratings.into_iter()
            .fold((0.0, 0.0), |(r_acc, s_acc), (r, s)| {
                (r*s + r_acc, s + s_acc)
            });

        if norm > 0.0 {
            Ok(rating / norm)
        }
        else {
            Err("No neighbors".to_string())
        }
    }
}
