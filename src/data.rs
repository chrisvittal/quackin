//! Module with basic data loading and handling utilities

use rusqlite::Connection;
use std::collections::{HashMap, HashSet};
use super::ID;
use csv::Reader;
use std::fs::File;

/// This trait is based on Mahout's `DataModel` interface.
/// States the basic functions that a repository containing information about
/// users, items and ratings must have. I'm still wondering if adding/removing
/// users and items should be on the trait or not
pub trait DataHandler {
    /// Returns all users IDs
    fn get_user_ids(&self) -> HashSet<ID>;
    /// Returns all items IDs
    fn get_item_ids(&self) -> HashSet<ID>;
    /// Returns the rating for each item rated by an user
    fn get_user_ratings(&self, user_id: ID) -> HashMap<ID, f64>;
    /// Returns the rating for each user who rated an item
    fn get_item_ratings(&self, item_id: ID) -> HashMap<ID, f64>;
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
    user_ids: HashSet<ID>,
    item_ids: HashSet<ID>,
    ratings: HashMap<(ID, ID), f64>
}

impl BasicDataHandler {
    /// Creates an empty data handler
    pub fn new() -> BasicDataHandler {
        let user_ids: HashSet<ID> = HashSet::new();
        let item_ids: HashSet<ID> = HashSet::new();
        let ratings: HashMap<(ID, ID), f64> = HashMap::new();
        BasicDataHandler {
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings
        }
    }
    /// Creates a data handler from a `csv::Reader`
    pub fn from_reader(mut reader: Reader<File>) -> BasicDataHandler {
        let mut user_ids: HashSet<ID> = HashSet::new();
        let mut item_ids: HashSet<ID> = HashSet::new();
        let mut ratings: HashMap<(ID, ID), f64> = HashMap::new();
        for row in reader.decode() {
            let (user_id, item_id, rating): (ID, ID, f64) = row.unwrap();
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

impl DataHandler for BasicDataHandler {
    fn get_user_ids(&self) -> HashSet<ID> {
        self.user_ids.clone()
    }
    fn get_item_ids(&self) -> HashSet<ID> {
        self.item_ids.clone()
    }
    /// This function is pretty slow, it needs to iterate over the whole rating
    /// matrix to recover the user ratings. We need a different data structure
    fn get_user_ratings(&self, user_id: ID) -> HashMap<ID, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.0 == user_id {
                return Some((k.1.clone(), *v));
            }
            return None;
        }).collect()
    }
    /// This function is pretty slow, it needs to iterate over the whole rating
    /// matrix to recover the user ratings. We need a different data structure
    fn get_item_ratings(&self, item_id: ID) -> HashMap<ID, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.1 == item_id {
                return Some((k.0.clone(), *v));
            }
            return None;
        }).collect()
    }
    fn get_rating(&self, user_id: ID, item_id: ID) -> f64 {
        *self.ratings.get(&(user_id, item_id)).unwrap_or(&-1.0)
    }
    fn get_num_users(&self) -> usize {
        self.user_ids.len()
    }
    fn get_num_items(&self) -> usize {
        self.item_ids.len()
    }
    fn add_user(&mut self, user_id: ID) -> bool {
        if !self.user_ids.contains(&user_id) {
            self.user_ids.insert(user_id);
            return true;
        }
        false
    }
    fn add_item(&mut self, item_id: ID) -> bool {
        if !self.item_ids.contains(&item_id) {
            self.item_ids.insert(item_id);
            return true;
        }
        false
    }
    fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool {
        if self.user_ids.contains(&user_id) &&
            self.item_ids.contains(&item_id) {
                self.ratings.insert((user_id, item_id), rating);
                return true;
            }
        false
    }
}

/// An Sqlite data handler, it should be faster than the BasicDataHandler
pub struct SqliteDataHandler {
    user_ids: HashSet<ID>,
    item_ids: HashSet<ID>,
    connection: Connection
}

impl SqliteDataHandler {
    /// Creates an empty data handler
    pub fn new() -> SqliteDataHandler {
        let connection = Connection::open_in_memory().unwrap();
         connection.execute("CREATE TABLE ratings (
                  user_id    INTEGER    NOT NULL,
                  item_id    INTEGER    NOT NULL,
                  rating     REAL       NOT NULL
                  )", &[]).unwrap();
        connection.execute("CREATE INDEX user_index ON ratings (user_id)", &[]).unwrap();
        connection.execute("CREATE INDEX item_index ON ratings (item_id)", &[]).unwrap();
        connection.execute("CREATE UNIQUE INDEX comp_index ON ratings (user_id, item_id)", &[]).unwrap();
        SqliteDataHandler {
            connection: connection,
            user_ids: HashSet::new(),
            item_ids: HashSet::new()
        }
    }
    /// Creates a data handler from a `csv::Reader`
    pub fn from_reader(mut reader: Reader<File>) -> SqliteDataHandler {
        let connection = Connection::open_in_memory().unwrap();
        let mut user_ids = HashSet::<ID>::new();
        let mut item_ids = HashSet::<ID>::new();
        connection.execute("CREATE TABLE ratings (
                  user_id    INTEGER    NOT NULL,
                  item_id    INTEGER    NOT NULL,
                  rating     REAL       NOT NULL
                  )", &[]).unwrap();
        connection.execute("CREATE INDEX user_index ON ratings (user_id)", &[]).unwrap();
        connection.execute("CREATE INDEX item_index ON ratings (item_id)", &[]).unwrap();
        connection.execute("CREATE UNIQUE INDEX comp_index ON ratings (user_id, item_id)", &[]).unwrap();
        for row in reader.decode() {
            let (user_id, item_id, rating): (i32, i32, f64) = row.unwrap();
            connection.execute("INSERT INTO ratings (user_id, item_id, rating)
                  VALUES (?1, ?2, ?3)", &[&user_id, &item_id, &rating]).unwrap();
            user_ids.insert(user_id as usize);
            item_ids.insert(item_id as usize);
        }
        SqliteDataHandler {
            connection: connection,
            user_ids: user_ids,
            item_ids: item_ids
        }
    }
}

impl DataHandler for SqliteDataHandler {
    fn get_user_ids(&self) -> HashSet<ID> {
        self.user_ids.clone()
    }
    fn get_item_ids(&self) -> HashSet<ID> {
        self.item_ids.clone()
    }
    fn get_user_ratings(&self, user_id: ID) -> HashMap<ID, f64> {
        let mut statement = self.connection
            .prepare("SELECT item_id, rating FROM ratings WHERE user_id = (?1)").unwrap();
        let rows = statement.query_map(&[&(user_id as i32)], |row| {
            (row.get(0), row.get(1))
        }).unwrap();
        let mut user_ratings = HashMap::<ID, f64>::new();
        for row in rows {
            let (item_id, rating): (i32, f64) = row.unwrap();
            user_ratings.insert(item_id as ID, rating);
        }
        user_ratings
    }
    fn get_item_ratings(&self, item_id: ID) -> HashMap<ID, f64> {
        let mut statement =self.connection
            .prepare("SELECT item_id, rating FROM ratings WHERE item_id = (?1)").unwrap();
        let rows = statement.query_map(&[&(item_id as i32)], |row| {
            (row.get(0), row.get(1))
        }).unwrap();
        let mut item_ratings = HashMap::<ID, f64>::new();
        for row in rows {
            let (user_id, rating): (i32, f64) = row.unwrap();
            item_ratings.insert(user_id as ID, rating);
        }
        item_ratings
    }
    fn get_rating(&self, user_id: ID, item_id: ID) -> f64 {
        let mut statement = self.connection
            .prepare("SELECT rating FROM ratings WHERE user_id = (?1) AND item_id = (?2)").unwrap();
        let mut rows = statement.query_map(&[&(user_id as i32), &(item_id as i32)], |row| {
            row.get(0)
        }).unwrap();
        rows.next().unwrap().unwrap()
    }
    fn get_num_users(&self) -> usize {
        self.user_ids.len()
    }
    fn get_num_items(&self) -> usize {
        self.item_ids.len()
    }
    fn add_user(&mut self, user_id: ID) -> bool {
        if !self.user_ids.contains(&user_id) {
            self.user_ids.insert(user_id);
            return true;
        }
        false
    }
    fn add_item(&mut self, item_id: ID) -> bool {
        if !self.item_ids.contains(&item_id) {
            self.item_ids.insert(item_id);
            return true;
        }
        false
    }
    fn add_rating(&mut self, user_id: ID, item_id: ID, rating: f64) -> bool {
        let transaction =
            self.connection.execute("INSERT INTO ratings (user_id, item_id, rating) VALUES (?1, ?2, ?3)",
                                    &[&(user_id as i32), &(item_id as i32), &rating]);
        match transaction {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
