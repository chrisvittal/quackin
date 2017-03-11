//! Module with the basic tools to build a recommender

use super::data::DataHandler;
use std::collections::HashMap;
use super::ID;


/// Trait that every recommender must satisfy.
pub trait Recommender {
    /// Predicts a rating given an user and an item.
    fn predict(&self, user: ID, item: ID) -> f64;
    /// Recommends items given an user. It must returns a vector of
    /// predicted ratings and item IDs sorted by rating
    fn recommend(&self, user_id: ID) -> Vec<(usize, f64)>;
}

pub struct BasicUserRecommender<'a, D: DataHandler + 'a> {
    data_handler: &'a mut D,
    threshold: f64,
    similarities: HashMap<(ID, ID), f64>,
    similarity: fn(&HashMap<ID, f64>, &HashMap<ID, f64>, usize) -> f64
}

impl<'a, D: DataHandler + 'a> BasicUserRecommender<'a, D> {
    pub fn new(data_handler: &mut D, threshold: f64, similarity: fn(&HashMap<ID, f64>, &HashMap<ID, f64>, usize) -> f64) -> BasicUserRecommender<D> {
        let mut similarities: HashMap<(ID, ID), f64> = HashMap::new();
        let user_ids = data_handler.get_user_ids();
        let n = data_handler.get_num_items();
        for user_id1 in &user_ids {
            for user_id2 in &user_ids {
                let user_1 = data_handler.get_user_ratings(*user_id1);
                let user_2 = data_handler.get_user_ratings(*user_id2);
                similarities.insert((*user_id1, *user_id2), similarity(&user_1, &user_2, n)); 
            }
        }
        BasicUserRecommender {
            data_handler: data_handler,
            threshold: threshold,
            similarities: similarities,
            similarity: similarity
        }
    }
    pub fn add_user(&mut self, user_id: ID) -> bool {
        self.data_handler.add_user(user_id)
    }
    pub fn add_item(&mut self, item_id: ID) -> bool {
        self.data_handler.add_item(item_id)
    }
    pub fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool {
        let result = self.data_handler.add_rating(user_id, item_id, rating);
        if result {
            let user_ids = self.data_handler.get_user_ids();
            let user_1 = self.data_handler.get_user_ratings(user_id);
            let n = self.data_handler.get_num_items();
            for user_id2 in &user_ids {
                let user_2 = self. data_handler.get_user_ratings(*user_id2);
                self.similarities.insert((user_id, *user_id2), (self.similarity)(&user_1, &user_2, n)); 
            }
        }
        result
    }
}

impl<'a, D: DataHandler + 'a> Recommender for BasicUserRecommender<'a, D> {
    fn predict(&self, user_id: ID, item_id: ID) -> f64 {
        let item = self.data_handler.get_item_ratings(item_id);
        let mut total_sim: f64 = 0.0;
        let mut total_rat: f64 = 0.0;
        for (other_id, rating) in item {
            let sim = *self.similarities.get(&(user_id, other_id)).unwrap();
            if sim > self.threshold {
                total_sim += sim;
                total_rat += rating*sim;
            }
        }
        if total_sim > 0.0 {
            return total_rat/total_sim;
        }
        0.0
    }
    fn recommend(&self, user_id: ID) -> Vec<(ID, f64)>{
        let items = self.data_handler.get_item_ids();
        let mut recom: Vec<(ID, f64)> = Vec::new();
        for item_id in items {
            let item = self.data_handler.get_item_ratings(item_id);
            let mut total_sim: f64 = 0.0;
            let mut total_rat: f64 = 0.0;
            for (other_id, rating) in item {
                let sim = *self.similarities.get(&(user_id, other_id)).unwrap();
                if sim > self.threshold {
                    total_sim += sim;
                    total_rat += rating*sim;
                }
            }
            if total_sim > 0.0 {
                recom.push((item_id, total_rat/total_sim));
            }
        }
        recom.sort_by(|&(id_1, _), &(id_2, _)| id_2.partial_cmp(&id_1).unwrap());
        recom
    }
}

pub struct BasicItemRecommender<'a, D: DataHandler + 'a> {
    data_handler: &'a mut D,
    threshold: f64,
    similarities: HashMap<(ID, ID), f64>,
    similarity: fn(&HashMap<ID, f64>, &HashMap<ID, f64>, usize) -> f64
}

impl<'a, D: DataHandler + 'a> BasicItemRecommender<'a, D> {
    pub fn new(data_handler: &mut D, threshold: f64, similarity: fn(&HashMap<usize, f64>, &HashMap<usize, f64>, usize) -> f64) -> BasicItemRecommender<D> {
        let mut similarities: HashMap<(ID, ID), f64> = HashMap::new();
        let item_ids = data_handler.get_item_ids();
        let n = data_handler.get_num_users();
        for item_id1 in &item_ids {
            for item_id2 in &item_ids {
                let item_1 = data_handler.get_item_ratings(*item_id1);
                let item_2 = data_handler.get_item_ratings(*item_id2);
                similarities.insert((*item_id1, *item_id2), similarity(&item_1, &item_2, n)); 
            }
        }
        BasicItemRecommender {
            data_handler: data_handler,
            threshold: threshold,
            similarities: similarities,
            similarity: similarity
        }
    }
    pub fn add_user(&mut self, user_id: ID) -> bool {
        self.data_handler.add_user(user_id)
    }
    pub fn add_item(&mut self, item_id: ID) -> bool {
        self.data_handler.add_item(item_id)
    }
    pub fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool {
        let result = self.data_handler.add_rating(user_id, item_id, rating);
        if result {
            let item_ids = self.data_handler.get_item_ids();
            let item_1 = self.data_handler.get_item_ratings(item_id);
            let n = self.data_handler.get_num_users();
            for item_id2 in &item_ids {
                let item_2 = self. data_handler.get_item_ratings(*item_id2);
                self.similarities.insert((item_id, *item_id2), (self.similarity)(&item_1, &item_2, n)); 
            }
        }
        result
    }
}

impl<'a, D: DataHandler + 'a> Recommender for BasicItemRecommender<'a, D> {
    fn predict(&self, user_id: ID, item_id: ID) -> f64 {
        let user = self.data_handler.get_user_ratings(user_id);
        let mut total_sim: f64 = 0.0;
        let mut total_rat: f64 = 0.0;
        for (other_id, rating) in user {
            let sim = *self.similarities.get(&(item_id, other_id)).unwrap();
            if sim > self.threshold {
                total_sim += sim;
                total_rat += rating*sim;
            }
        }
        if total_sim > 0.0 {
            return total_rat/total_sim;
        }
        0.0
    }
    
    fn recommend(&self, user_id: ID) -> Vec<(ID, f64)>{
        let user = self.data_handler.get_user_ratings(user_id);
        let items = self.data_handler.get_item_ids();
        let mut recom: Vec<(ID, f64)> = Vec::new();
        for item_id in items {
            let mut total_sim: f64 = 0.0;
            let mut total_rat: f64 = 0.0;
            for (other_id, rating) in &user {
                let sim = *self.similarities.get(&(item_id, *other_id)).unwrap();
                if sim > self.threshold {
                    total_sim += sim;
                    total_rat += rating*sim;
                }
            }
            if total_sim > 0.0 {
                recom.push((item_id, total_rat/total_sim));
            }
        }
        recom.sort_by(|&(id_1, _), &(id_2, _)| id_2.partial_cmp(&id_1).unwrap());
        recom
    }
}
