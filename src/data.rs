//! Module with basic data loading and handling utilities
use std::collections::BTreeMap;
use super::ID;
use csv::Reader;
use std::fs::File;

/// This trait is based on Mahout's `DataModel` interface.
/// States the basic functions that a repository containing information about
/// users, items and ratings must have. I'm still wondering if adding/removing
/// users and items should be on the trait or not
pub trait DataHandler {
    /// Returns all users IDs
    fn get_user_ids(&self) -> Vec<ID>;
    /// Returns all items IDs
    fn get_item_ids(&self) -> Vec<ID>;
    /// Returns the rating for each item rated by an user
    fn get_user_ratings(&self, user_id: ID) -> BTreeMap<ID, f64>;
    /// Returns the rating for each user who rated an item
    fn get_item_ratings(&self, item_id: ID) -> BTreeMap<ID, f64>;
    /// Rturns the rating given by an user to an item
    fn get_rating(&self, user_id: ID, item_id: ID) -> f64;
    /// Returns the number of users
    fn get_num_users(&self) -> usize;
    /// Returns the number of items
    fn get_num_items(&self) -> usize;
    /// Adds a new user, it returns `true` if the used was added
    fn add_user(&mut self, user_id: ID) -> bool;
    /// Adds a new item, it returns `true` if the item was added
    fn add_item(&mut self, item_id: ID) -> bool;
    /// Adds a rating given by an user to an item, it returns `true` if the
    ///rating was added
    fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool;
}

/// A basic data handler, it should be enough almost everytime or
/// it can be used as an example if you want to write your own
pub struct BasicDataHandler {
    user_ratings: BTreeMap<ID, BTreeMap<ID, f64>>,
    item_ratings: BTreeMap<ID, BTreeMap<ID, f64>>
}

impl BasicDataHandler {
    /// Creates an empty data handler
    pub fn new() -> BasicDataHandler {
        let user_ratings: BTreeMap<ID, BTreeMap<ID, f64>> = BTreeMap::new();
        let item_ratings: BTreeMap<ID, BTreeMap<ID, f64>> = BTreeMap::new();
        BasicDataHandler {
            user_ratings: user_ratings,
            item_ratings: item_ratings
        }
    }
    /// Creates a data handler from a `csv::Reader`
    pub fn from_reader(mut reader: Reader<File>) -> BasicDataHandler {
        let mut user_ratings: BTreeMap<ID, BTreeMap<ID, f64>> = BTreeMap::new();
        let mut item_ratings: BTreeMap<ID, BTreeMap<ID, f64>> = BTreeMap::new();
        for row in reader.decode() {
            let (user_id, item_id, rating): (ID, ID, f64) = row.unwrap();
            
            if user_ratings.contains_key(&user_id) {
                let user_rating = user_ratings.get_mut(&user_id).unwrap();
                user_rating.insert(item_id, rating);
            }
            else {
                let mut user_rating: BTreeMap<ID, f64> = BTreeMap::new();
                user_rating.insert(item_id, rating);
                user_ratings.insert(user_id, user_rating);
            }

            if item_ratings.contains_key(&item_id) {
                let item_rating = item_ratings.get_mut(&item_id).unwrap();
                item_rating.insert(user_id, rating);
            }
            else {
                let mut item_rating: BTreeMap<ID, f64> = BTreeMap::new();
                item_rating.insert(user_id, rating);
                item_ratings.insert(item_id, item_rating);
            }
        }
        BasicDataHandler {
            user_ratings: user_ratings,
            item_ratings: item_ratings
        }
    }
}

impl DataHandler for BasicDataHandler {
    fn get_user_ids(&self) -> Vec<ID> {
        self.user_ratings.keys().cloned().collect()
    }
    fn get_item_ids(&self) -> Vec<ID> {
        self.item_ratings.keys().cloned().collect()
    }
    fn get_user_ratings(&self, user_id: ID) -> BTreeMap<ID, f64> {
        self.user_ratings.get(&user_id).unwrap().clone()
    }
    fn get_item_ratings(&self, item_id: ID) -> BTreeMap<ID, f64> {
        self.item_ratings.get(&item_id).unwrap().clone()
    }
    fn get_rating(&self, user_id: ID, item_id: ID) -> f64 {
        *self.user_ratings.get(&user_id).unwrap().get(&item_id).unwrap_or(&-1.0)
    }
    fn get_num_users(&self) -> usize {
        self.user_ratings.len()
    }
    fn get_num_items(&self) -> usize {
        self.item_ratings.len()
    }
    fn add_user(&mut self, user_id: ID) -> bool {
        if !self.user_ratings.contains_key(&user_id) {
            self.user_ratings.insert(user_id, BTreeMap::new());
            return true;
        }
        false
    }
    fn add_item(&mut self, item_id: ID) -> bool {
        if !self.item_ratings.contains_key(&item_id) {
            self.item_ratings.insert(item_id, BTreeMap::new());
            return true;
        }
        false

    }
    fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool {
        if self.user_ratings.contains_key(&user_id) && self.item_ratings.contains_key(&item_id) {
            let user_rating = self.user_ratings.get_mut(&user_id).unwrap();
            let item_rating = self.item_ratings.get_mut(&item_id).unwrap();
            user_rating.insert(item_id, rating);
            item_rating.insert(user_id, rating);
            return true;
        }
        false
    }
}
