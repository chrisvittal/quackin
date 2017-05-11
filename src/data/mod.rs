//! This submodule provides tools related to reading and handling data.
//!
//! Given that the most popular datasets for recommender systems are stored
//! as csv files, that's the only supported format at the moment.
//!
//! # Examples
//!
//! ```ignore
//! use quackin::data::{DefaultRecord, read_ratings}
//!
//! let records: Vec<DefaultRecord> = read_ratings("path/to/file", None, true);
//! //                                                             ^^^^  ^^^^
//! //                                                             |     |
//! //                                                             |     the file has headers
//! //                                                             use ',' as separator
//! for record in records {
//!     println!("{} {} {}", record.user_id, record.item_id, record.rating);
//! }
use csv;
use rustc_serialize::Decodable;


/// A record consisting only of an `user_id`, an `item_id` and a `rating`
#[derive(RustcDecodable)]
pub struct BaseRecord<U, I> {
    user_id: U,
    item_id: I,
    rating: f64,
}

/// A `BaseRecord` where the user_id and item_id are of type `String`
pub type DefaultRecord = BaseRecord<String, String>;

/// Reads a csv file and loads its contents into a `Vec` of records.
///
/// `delimiter` defines if a delimiter must be used when reading the csv file,
/// if is `None` it uses a `,` as default. `has_headers` defines if the csv file
/// has headers or not.
///
/// Currently this function assumes that the records are stored on an arbitrary
///  `struct` because there is no way of dinamically setting the number of
/// columns nor the order of these. This needs refinement, but it works.
pub fn read_ratings<R>(path: &str, delimiter: Option<char>, has_headers: bool) -> Result<Vec<R>, csv::Error> where R: Decodable {
    let del = match delimiter {
        Some(del) => del as u8,
        None => ',' as u8
    };

    let mut reader = try!(csv::Reader::from_file(path)).has_headers(has_headers).delimiter(del);
    let ratings = reader.decode().map(|record| {record.unwrap()}).collect::<Vec<R>>();

    Ok(ratings)
}
