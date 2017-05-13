//! This submodule provides tools related to reading and handling data.
//!
//! Given that the most popular datasets for recommender systems are stored
//! as csv files, that's the only supported format at the moment.
//!
//! # Examples
//!
//! ```ignore
//! use quackin::data::{Record, ReadOptions, read_records}
//!
//! let records: Vec<Record> = read_records("path/to/file", ReadOptions::default());
//!
//! for record in records {
//!     println!("{:?}", record);
//! }
//! ```
use csv;


/// Type for a record, it is a tuple of two `Strings` and a `f64`
///
/// It represents a record of a dataset consisting of:
///
/// - An user ID
/// - An item ID
/// - A rating
pub type Record = (String, String, f64);

/// Possible fields on a dataset
pub enum Field {
    UserID,
    ItemID,
    Rating,
    Other
}

/// Options when reading a csv file
pub struct ReadOptions {
    fields: Vec<Field>,
    has_headers: bool,
    delimiter: char
}

impl ReadOptions {
    /// Assumes the csv file has no headers, uses `','` as delimiter
    /// and that the columns are in the order `UserID`, `ItemID`, `Rating`
    pub fn default() -> Self {
        Self {
            fields: vec![Field::UserID, Field::ItemID, Field::Rating],
            has_headers: false,
            delimiter: ','
        }
    }

    /// Constructor for custom options, use it if the columns of the csv file
    /// are not in the `default` order, or if you need to specify the delimiter
    /// or the presence of headers in the file.
    ///
    /// ## Example
    ///
    /// If you have a csv file like this:
    ///
    /// ```text, no_run
    /// item_id user_id timestamp rating
    /// u_00001 i_00001 765456787 5.0
    /// u_00002 i_00002 534623443 2.0
    /// ...
    /// ```
    /// you will need to use the `ReadOptions::custom` constructor like this
    ///
    /// ```rust
    /// use quackin::data::ReadOptions;
    /// use quackin::data::Field::*;
    ///
    /// let options = ReadOptions::custom(vec![ItemID, UserID, Other, Rating], true, ' ');
    /// ```
    pub fn custom(fields: Vec<Field>, has_headers: bool, delimiter: char) -> Self {
        Self {
            fields: fields,
            has_headers: has_headers,
            delimiter: delimiter
        }
    }
}

/// Possible errors when reading a dataset
#[derive(Debug)]
pub enum ReadError {
    Other(&'static str),
    Csv(csv::Error),
}

impl From<csv::Error> for ReadError {
    fn from(err: csv::Error) -> ReadError {
        ReadError::Csv(err)
    }
}

/// Reads the records from a dataset stored in a csv file with custom options.
///
/// The first parameter is just the path where the csv file is located as a
/// `&str`. The second parameter consists of an `struct` of type `ReadOptions`
/// for custom reading options.
pub fn read_custom_records(path: &str, options: ReadOptions) -> Result<Vec<Record>, ReadError> {
    let n_fields = options.fields.len();

    let mut user_index: usize = n_fields;
    let mut item_index: usize = n_fields;
    let mut rating_index: usize = n_fields;

    for i in 0..n_fields {
        match options.fields[i] {
            Field::UserID => user_index = i,
            Field::ItemID => item_index = i,
            Field::Rating => rating_index = i,
            Field::Other => ()
        }
    }

    if [user_index, item_index, rating_index].iter().any(|&x| x == n_fields) {
        return Err(ReadError::Other("Unconsistent field format"));
    }

    let del = options.delimiter as u8;

    let mut reader = try!(csv::Reader::from_file(path))
        .has_headers(options.has_headers)
        .delimiter(del);
    let ratings = reader.decode().map(|row| {
        let row: Vec<String> = row.unwrap();
        (row[user_index].clone(),
         row[item_index].clone(),
         row[rating_index].parse::<f64>().unwrap())
    }).collect::<Vec<(String, String, f64)>>();

    Ok(ratings)
}

/// Reads the records from a dataset stored in a csv file.
pub fn read_records(path: &str) -> Result<Vec<Record>, ReadError> {
    read_custom_records(path, ReadOptions::default())
}
