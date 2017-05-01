mod data_set;

pub use self::data_set::DataSet;

use std::collections::HashMap;
use std::fs::File;
use sprs::CsVecOwned;
use csv::Reader;

pub struct BasicDataSet {
    user_indices: HashMap<String, usize>,
    item_indices: HashMap<String, usize>,
    ratings: HashMap<usize, f64>
}

impl BasicDataSet {
    pub fn from_csv(mut users_reader: Reader<File>, mut items_reader: Reader<File>, mut ratings_reader: Reader<File>) -> Self {
        let mut user_index = 0;
        let mut item_index = 0;
        let mut user_indices = HashMap::new();
        let mut item_indices = HashMap::new();
        let mut ratings = HashMap::new();

        for row in users_reader.decode() {
            let user_id: String = row.unwrap();
            user_indices.insert(user_id, user_index);
            user_index += 1;
        }

        for row in items_reader.decode() {
            let item_id: String = row.unwrap();
            item_indices.insert(item_id, item_index);
            item_index += 1;
        }

        for row in ratings_reader.decode() {
            let (user_id, item_id, rating): (String, String, f64) = row.unwrap();
            user_index = *user_indices.get(&user_id).expect("Invalid User ID");
            item_index = *item_indices.get(&item_id).expect("Invalid Item ID");
            ratings.insert(user_index * item_indices.len() + item_index, rating);
        }

        Self {
            user_indices: user_indices,
            item_indices: item_indices,
            ratings: ratings,
        }
    }
}

impl DataSet for BasicDataSet {
    fn get_user_indices(&self) -> HashMap<String, usize> {
        self.user_indices.clone()
    }
    fn get_item_indices(&self) -> HashMap<String, usize> {
        self.item_indices.clone()
    }
    fn get_user_vectors(&self) -> HashMap<String, CsVecOwned<f64>> {
        let mut user_vectors = HashMap::new();
        let n_items = self.item_indices.len();
        for (user_id, user_index) in self.user_indices.iter() {
            let mut data = Vec::new();
            let mut ind = Vec::new();
            for item_index in 0..n_items {
                let rating_index = item_index + user_index*n_items;
                if let Some(rating) = self.ratings.get(&rating_index) {
                    data.push(*rating);
                    ind.push(item_index);
                }
            }
            let user_vector = CsVecOwned::new(n_items, ind, data);
            user_vectors.insert(user_id.clone(), user_vector);
        }
        user_vectors
    }
    fn get_item_vectors(&self) -> HashMap<String, CsVecOwned<f64>> {
        let mut item_vectors = HashMap::new();
        let n_items = self.item_indices.len();
        let n_users = self.user_indices.len();
        for (item_id, item_index) in self.item_indices.iter() {
            let mut data = Vec::new();
            let mut ind = Vec::new();

            for user_index in 0..n_users {
                let rating_index = item_index + user_index*n_items;
                if let Some(rating) = self.ratings.get(&rating_index) {
                    data.push(*rating);
                    ind.push(user_index);
                }
            }

            let item_vector = CsVecOwned::new(n_users, ind, data);
            item_vectors.insert(item_id.clone(), item_vector);
        }
        item_vectors
    }
}
