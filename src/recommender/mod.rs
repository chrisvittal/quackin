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

use std::hash::Hash;
use std::collections::HashMap;

use sprs::CsVecOwned;
use rustc_serialize::Decodable;

use super::data::Record;
use super::metrics::similarity::Similarity;

pub struct KnnUserRecommender<U, I> {
    user_indices: HashMap<U, usize>,
    item_indices: HashMap<I, usize>,
    user_ids: Vec<U>,
    item_ids: Vec<I>,
    ratings: HashMap<usize, CsVecOwned<f64>>,
    similarity: Similarity,
}

impl<U, I> KnnUserRecommender<U, I> where U: Hash + Eq + Decodable + Clone, I: Hash + Eq + Decodable + Clone {
    pub fn from_records<R>(records: &[R], similarity: Similarity) -> Self where R: Record<U, I> {
        let mut user_indices = HashMap::<U, usize>::new();
        let mut item_indices = HashMap::<I, usize>::new();
        let mut user_ids = Vec::new();
        let mut item_ids = Vec::new();
        let mut pre_vectors = HashMap::<usize, (Vec<usize>, Vec<f64>)>::new();
        let (mut i, mut j) = (0, 0);

        for record in records {
            let user_id = record.get_user_id();
            let item_id = record.get_item_id();
            let rating = record.get_rating();

            if !(user_ids.contains(user_id)) {
                user_ids.push(user_id.clone());
                user_indices.insert(user_id.clone(), i);
                i += 1;
            }

            if !(item_ids.contains(item_id)) {
                item_ids.push(item_id.clone());
                item_indices.insert(item_id.clone(), j);
                j += 1;
            }

            let user_index = user_indices[&user_id];
            let item_index = item_indices[&item_id];

            if pre_vectors.contains_key(&user_index) {
                let pre_vector = pre_vectors.get_mut(&user_index).unwrap();
                pre_vector.0.push(item_index);
                pre_vector.1.push(rating);
            }
            else {
                pre_vectors.insert(user_index, (vec![item_index], vec![rating]));
            }
        }

        let ratings = pre_vectors.into_iter()
            .map(|(k, (ind, dat))| (k, CsVecOwned::new(item_indices.len(), ind, dat)))
            .collect();
        Self {
            user_indices: user_indices,
            item_indices: item_indices,
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings,
            similarity: similarity,
        }
    }

    pub fn predict(&self, user_id: &U, item_id: &I, k: usize) -> Result<f64, String> {
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
        ratings.truncate(k);


        let (rating, norm) = ratings.into_iter()
            .fold((0.0, 0.0), |(r_acc, s_acc), (r, s)| {
                (r*s + r_acc, s + s_acc)
            });

        if norm > 0.0 {
            Ok(rating / norm)
        }
        else {
            Ok(0.0)
        }
    }
}
