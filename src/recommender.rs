//! Module with the basic tools to build a recommender

use super::data::DataHandler;
use std::collections::HashMap;
use super::ID;


/// Trait that every recommender must satisfy.
pub trait Recommender {
    /// Predicts a rating given an user and an item.
    fn predict(&self, user_id: ID, item_id: ID) -> f64;
    /// Recommends items given an user. It must returns a vector of
    /// predicted ratings and item IDs sorted by rating
    fn recommend(&self, user_id: ID) -> Vec<(usize, f64)>;
}

/// User based nearest neighbors recommender
pub struct NearestUserRecommender<'a, D: DataHandler + 'a> {
    data_handler: &'a D,
    k: usize,
    similarity: fn(&HashMap<usize, f64>, &HashMap<usize, f64>, usize) -> f64
}

impl<'a, D: DataHandler + 'a> NearestUserRecommender<'a, D> {
    /// Creates an user based nearest neighbors recommender given a data
    /// handler, a similarity and the number of neighbors
    pub fn new(data_handler: &D, similarity: fn(&HashMap<usize, f64>, &HashMap<usize, f64>, usize) -> f64, k: usize) -> NearestUserRecommender<D> {
        NearestUserRecommender {
            data_handler: data_handler,
            k: k,
            similarity: similarity
        }
    }
    /// returns the nearest neighbors for a given user, it requires the number
    /// of items to compute the similarity
    pub fn get_neighbors(&self, user: HashMap<usize, f64>, n: usize) -> Vec<(f64, HashMap<usize, f64>)> {
        let mut neighbors: Vec<(f64, HashMap<usize, f64>)> =
            self.data_handler.get_user_ids().iter().map(|x|{
                let other = self.data_handler.get_user_ratings(*x);
                ((self.similarity)(&user, &other, n), other)
            }).collect();
        neighbors.sort_by(|x,y| y.0.partial_cmp(&x.0).unwrap());
        neighbors.remove(0);
        neighbors.truncate(self.k);
        neighbors
    }
}

impl<'a, D: DataHandler + 'a> Recommender for NearestUserRecommender<'a, D> {
    /// Predicts the rating for an item by an user. It returns zero if there are
    /// no neighbors with a similarity greater then zero
    fn predict(&self, user_id: ID, item_id: ID) -> f64 {
        let n = self.data_handler.get_num_items();
        let user = self.data_handler.get_user_ratings(user_id);
        let neighbors: Vec<(f64, HashMap<usize, f64>)> =
            self.get_neighbors(user, n).into_iter().filter(|x| {
                x.1.contains_key(&item_id)
            }).collect();
        let mut total_sim: f64 = 0.0;
        let mut rating: f64 = 0.0;
        for (sim, other) in neighbors {
            total_sim += sim;
            rating += sim*other.get(&item_id).unwrap();
        }
        if total_sim > 0.0 {
            return rating/total_sim;
        }
        0.0
    }
    /// Returns a vector of item IDs sorted by the predicted rating
    fn recommend(&self, user_id: ID) -> Vec<(usize, f64)>{
        let n = self.data_handler.get_num_items();
        let user = self.data_handler.get_user_ratings(user_id);
        let neighbors = self.get_neighbors(user.clone(), n);
        let mut ratings: Vec<(usize, f64)> = Vec::with_capacity(n);
        let items = self.data_handler.get_item_ids().into_iter().filter(|x| {
            !&user.contains_key(&x)
        });
        for item_id in items {
            let mut total_sim: f64 = 0.0;
            let mut rating: f64 = 0.0;
            for &(sim, ref other) in neighbors.iter().filter(|x| {
                x.1.contains_key(&item_id)
            }) {
                total_sim += sim;
                rating += sim*other.get(&item_id).unwrap();
            }
            if total_sim > 0.0 {
                rating = rating/total_sim;
            }
            else {
                rating = 0.0;
            }
            ratings.push((item_id, rating));
        }
        ratings.sort_by(|x,y| y.1.partial_cmp(&x.1).unwrap());
        ratings
    }
}
