extern crate csv;
extern crate oozie;

use oozie::data::{BasicDataSet, DataSet};
use csv::Reader;

#[test]
fn load_basic_data_set_test() {
    let user_reader = Reader::from_file("data/mini_set/users.csv").unwrap();
    let item_reader = Reader::from_file("data/mini_set/items.csv").unwrap();
    let rating_reader = Reader::from_file("data/mini_set/ratings.csv").unwrap();

    let data_set = BasicDataSet::from_csv(user_reader, item_reader, rating_reader);
    println!("{:?}", data_set.get_user_indices());
    println!("{:?}", data_set.get_item_indices());
    println!("{:?}", data_set.get_user_vectors());
    println!("{:?}", data_set.get_item_vectors());
}
