//! This submodule provides tools related to reading and handling data.
//!
//! Given that the most popular datasets for recommender systems are stored
//! as csv files, that's the only supported format at the moment.
//!
//! # Examples
//!
//! ```ignore
//! use quackin::data::{DefaultRecord, read_records}
//!
//! let records: Vec<DefaultRecord> = read_records("path/to/file", None, true);
//! //                                                             ^^^^  ^^^^
//! //                                                             |     |
//! //                                                             |     the file has headers
//! //                                                             use ',' as separator
//! for record in records {
//!     println!("{} {} {}", record.user_id, record.item_id, record.rating);
//! }
use csv;
use std::hash::Hash;
use rustc_serialize::Decodable;

/// Trait that every record must satisfy.
///
/// This is intended to be used when a dataset has a order in its columns
/// different from the one of `BaseRecord` which is `user_id,item_id,rating`
/// or when the dataset has more columns.
///
/// It would be cool to add a derive for this trait.
///
/// # Examples
///
/// Lets suppose that we have a dataset with the following columns:
/// `user_id,product_name,rating,timestamp`, we will write a struct for
/// decoding such data.
///
/// ```ignore
/// use rustc_serialize::Decodable;
/// use quackin::data::Record;
///
/// #[derive(RustcDecodable)]
/// struct MyRecord {
///     user_id: u32,
///     product_name: String, // fields can have any name
///     rating: f64,
///     timestamp: u64, // we can have additional fields
/// }
///
/// impl Record<u32, String> for MyRecord {
///     fn get_user_id(&self) -> &u32 {
///         &self.user_id
///     }
///     fn get_item_id(&self) -> &String {
///         &self.product_name
///     }
///     fn get_rating(&self) -> f64 {
///         self.rating
///     }
/// }
///
/// // Now we can read the records
/// let my_records: Vec<MyRecord> = read_records("path/to/dataset", None, false).unwrap();
/// ```
pub trait Record<U, I>: Decodable where U: Hash + Eq + Decodable, I: Hash + Eq + Decodable {
    fn get_user_id(&self) -> &U;
    fn get_item_id(&self) -> &I;
    fn get_rating(&self) -> f64;
}

/// A record consisting only of an `user_id`, an `item_id` and a `rating`
#[derive(RustcDecodable)]
pub struct BaseRecord<U, I> {
    user_id: U,
    item_id: I,
    rating: f64,
}

impl<U, I> Record<U, I> for BaseRecord<U, I> where U: Hash + Eq + Decodable, I: Hash + Eq + Decodable {
    fn get_user_id(&self) -> &U {
        &self.user_id
    }
    fn get_item_id(&self) -> &I {
        &self.item_id
    }
    fn get_rating(&self) -> f64 {
        self.rating
    }
}

/// A `BaseRecord` where `user_id` and `item_id` are of type `String`
pub type DefaultRecord = BaseRecord<String, String>;

/// Reads a csv file and loads its contents into a `Vec` of records.
///
/// `delimiter` defines if a delimiter must be used when reading the csv file,
/// if is `None` it uses a `,` as default. `has_headers` defines if the csv file
/// has headers or not.
///
/// Currently this function assumes that the records are stored on an `struct`
/// that implements `Record` because there is no way of dinamically setting the
/// number of columns nor the order of these. This needs refinement, but it
/// works.
pub fn read_records<R, U, I>(path: &str, delimiter: Option<char>, has_headers: bool) -> Result<Vec<R>, csv::Error> where R: Record<U, I>, U: Hash + Eq + Decodable, I: Hash + Eq + Decodable {
    let del = match delimiter {
        Some(del) => del as u8,
        None => ',' as u8
    };

    let mut reader = try!(csv::Reader::from_file(path)).has_headers(has_headers).delimiter(del);
    let ratings = reader.decode().map(|record| record.unwrap()).collect::<Vec<R>>();

    Ok(ratings)
}
