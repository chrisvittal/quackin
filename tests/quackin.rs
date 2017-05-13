#![allow(dead_code)]
#![allow(unused_variables)]

extern crate quackin;

use quackin::data::{Record, ReadOptions, read_records, read_custom_records};
use quackin::data::Field::*;

use quackin::recommender::KnnUserRecommender;
use quackin::metrics::similarity::cosine;

#[test]
fn read_default_file_test() {
    let records = read_records("data/mock.csv").unwrap();
}

#[test]
fn read_file_with_headers_test() {
    let options = ReadOptions::custom(vec![UserID, ItemID, Rating], true, ',');
    let records = read_custom_records("data/mock_headers.csv", options).unwrap();
}

#[test]
fn read_file_with_custom_separator_test() {
    let options = ReadOptions::custom(vec![UserID, ItemID, Rating], true, '?');
    let records = read_custom_records("data/mock_separator.csv", options).unwrap();
}

#[test]
fn read_file_with_more_columns_test() {
    let options = ReadOptions::custom(vec![UserID, ItemID, Other, Rating], false, ',');
    let records = read_custom_records("data/mock_custom.csv", options).unwrap();
}

#[test]
fn knn_user_recommender_test() {
    let records = read_records("data/mock.csv").unwrap();
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
