extern crate oozie;
extern crate rustc_serialize;

use oozie::data::{DefaultRecord, read_ratings};

#[derive(RustcDecodable)]
pub struct CustomRecord {
    item_id: i32,
    user_id: i32,
    rating: f64,
    stuff: i32
}

#[test]
fn read_mock() {
    read_ratings::<DefaultRecord>("data/mock.csv", None, false).unwrap();
}

#[test]
fn read_mock_with_headers() {
    read_ratings::<DefaultRecord>("data/mock_headers.csv", None, true).unwrap();
}

#[test]
fn read_mock_with_separator() {
    read_ratings::<DefaultRecord>("data/mock_separator.csv", Some('?'), true).unwrap();
}

#[test]
fn read_mock_with_custom_records() {
    read_ratings::<CustomRecord>("data/mock_custom.csv", None, false).unwrap();
}
