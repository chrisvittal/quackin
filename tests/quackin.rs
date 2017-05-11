extern crate quackin;
extern crate rustc_serialize;

use quackin::data::{DefaultRecord, Record, read_records};
use quackin::recommender::KnnUserRecommender;
use quackin::metrics::similarity::cosine;

#[derive(RustcDecodable)]
pub struct CustomRecord {
    item_id: i32,
    user_id: i32,
    rating: f64,
    stuff: i32
}

impl Record<i32, i32> for CustomRecord {
    fn get_user_id(&self) -> &i32 {
        &self.user_id
    }
    fn get_item_id(&self) -> &i32 {
        &self.item_id
    }
    fn get_rating(&self) -> f64 {
        self.rating
    }
}

#[test]
fn read_mock() {
    let records: Vec<DefaultRecord> = read_records("data/mock.csv", None, false).unwrap();
}

#[test]
fn read_mock_with_headers() {
    let records: Vec<DefaultRecord> = read_records("data/mock_headers.csv", None, true).unwrap();
}

#[test]
fn read_mock_with_separator() {
    let records: Vec<DefaultRecord> = read_records("data/mock_separator.csv", Some('?'), true).unwrap();
}

#[test]
fn read_mock_with_custom_records() {
    let records: Vec<CustomRecord> = read_records("data/mock_custom.csv", None, false).unwrap();
}

#[test]
fn load_mock_into_knn_user() {
    let records: Vec<DefaultRecord> = read_records("data/mock.csv", None, false).unwrap();
    let recommender = KnnUserRecommender::from_records(&records, cosine);
    println!("{:?}", recommender.predict(&"user_1".to_string(), &"item_1".to_string(), 10));
    println!("{:?}", recommender.predict(&"user_2".to_string(), &"item_2".to_string(), 10));
    println!("{:?}", recommender.predict(&"user_3".to_string(), &"item_3".to_string(), 10));
    println!("{:?}", recommender.predict(&"user_4".to_string(), &"item_4".to_string(), 10));
    println!("{:?}", recommender.predict(&"user_5".to_string(), &"item_5".to_string(), 10));
}
