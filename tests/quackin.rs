#![allow(dead_code)]
#![allow(unused_variables)]
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
fn knn_user_recommender() {
    let records: Vec<DefaultRecord> = read_records("data/mock.csv", None, false).unwrap();
    let recommender = KnnUserRecommender::from_records(&records, cosine, 5);

    let some_uir = vec![("user_2", "item_3", 2.5192531497347637),
                        ("user_1", "item_3", 2.9524340130950657),
                        ("user_6", "item_3", 2.767575112334526),
                        ("user_4", "item_3", 2.7332710059168677),
                        ("user_5", "item_3", 2.7369426258734384),
                        ("user_8", "item_3", 2.9612309722134706),
                        ("user_9", "item_3", 2.458585213496907)]
        .into_iter()
        .map(|(x, y, z)| (x.to_string(), y.to_string(), z));

    for (user_id, item_id, rating) in some_uir {
        let pred_rat = recommender.predict(&user_id, &item_id).expect("Should be possible to compute rating");
        assert!((pred_rat - rating).abs() < 0.1);
    }
}
