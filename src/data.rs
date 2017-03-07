//! Basic data loading and handling utilities

use std::fs::File;
use std::collections::{HashMap, HashSet};
use csv::Reader;
use super::Key;

/// This trait is based on Mahout's `DataModel` interface.
/// States the basic functions that a repository containing information about users, items and ratings must have.
/// I'm still wondering if adding/removing users and items should be on the trait or not
pub trait DataHandler<U, I> {
    /// Returns all users IDs
    fn get_user_ids(&self) -> &HashSet<U>;
    /// Returns all items IDs
    fn get_item_ids(&self) -> &HashSet<I>;
    /// Returns the rating for each item rated by an user
    fn get_user_ratings(&self, user_id: U) -> HashMap<I, f64>;
    /// Returns the rating for each user who rated an item
    fn get_item_ratings(&self, item_id: I) -> HashMap<U, f64>;
    /// Rturns the rating given by an user to an item
    fn get_rating(&self, user_id: U, item_id: I) -> f64;
    /// Returns the number of users
    fn get_num_users(&self) -> usize;
    /// Returns the number of items
    fn get_num_items(&self) -> usize;
    /// Adds a new user, it returns `true` if the used was added
    fn add_user(&mut self, user_id: U) -> bool;
    /// Adds a new item, it returns `true` if the item was added
    fn add_item(&mut self, item_id: I) -> bool;
    /// Adds a rating given by an user to an item, it returns `true` if the rating was added
    fn add_rating(&mut self, user_id: U, item_id: I, rating: f64) -> bool;
    /// Removes a rating given by an user to an item
    fn remove_rating(&mut self, user_id: U, item_id: I);
}

/// A basic data handler, it should be enough almost everytime or
/// it can be used as an example if you want to write your own
pub struct BasicDataHandler<U: Key, I: Key> {
    user_ids: HashSet<U>,
    item_ids: HashSet<I>,
    ratings: HashMap<(U, I), f64>
}

impl<U: Key, I: Key> BasicDataHandler<U, I> {
    /// Creates an empty data handler
    pub fn new() -> BasicDataHandler<U, I> {
        let user_ids: HashSet<U> = HashSet::new();
        let item_ids: HashSet<I> = HashSet::new();
        let ratings: HashMap<(U, I), f64> = HashMap::new();
        BasicDataHandler {
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings
        }
    }
    /// Creates a data handler from a `csv::Reader`
    pub fn from_reader(mut reader: Reader<File>) -> BasicDataHandler<U, I> {
        let mut user_ids: HashSet<U> = HashSet::new();
        let mut item_ids: HashSet<I> = HashSet::new();
        let mut ratings: HashMap<(U, I), f64> = HashMap::new();
        for row in reader.decode() {
            let (user_id, item_id, rating): (U, I, f64) = row.unwrap();
            user_ids.insert(user_id.clone());
            item_ids.insert(item_id.clone());
            ratings.insert((user_id, item_id), rating);
        }
        BasicDataHandler {
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings
        }
    }
}

impl<U: Key, I: Key> DataHandler<U, I> for BasicDataHandler<U, I> {
    fn get_user_ids(&self) -> &HashSet<U> {
        &self.user_ids
    }
    fn get_item_ids(&self) -> &HashSet<I> {
        &self.item_ids
    }
    fn get_user_ratings(&self, user_id: U) -> HashMap<I, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.0 == user_id {
                return Some((k.1.clone(), *v));
            }
            return None;
        }).collect()
    }
    fn get_item_ratings(&self, item_id: I) -> HashMap<U, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.1 == item_id {
                return Some((k.0.clone(), *v));
            }
            return None;
        }).collect()
    }
    fn get_rating(&self, user_id: U, item_id: I) -> f64 {
        *self.ratings.get(&(user_id, item_id)).unwrap_or(&-1.0)
    }
    fn get_num_users(&self) -> usize {
        self.user_ids.len()
    }
    fn get_num_items(&self) -> usize {
        self.item_ids.len()
    }
    fn add_user(&mut self, user_id: U) -> bool {
        if !self.user_ids.contains(&user_id) {
            self.user_ids.insert(user_id);
            return true;
        }
        false
    }
    fn add_item(&mut self, item_id: I) -> bool {
        if !self.item_ids.contains(&item_id) {
            self.item_ids.insert(item_id);
            return true;
        }
        false
    }
    fn add_rating(&mut self, user_id: U, item_id: I, rating: f64) -> bool {
        if self.user_ids.contains(&user_id) && self.item_ids.contains(&item_id) {
            self.ratings.insert((user_id, item_id), rating);
            return true;
        }
        false
    }
    fn remove_rating(&mut self, user_id: U, item_id: I) {
        self.ratings.remove(&(user_id, item_id));
    }
}
