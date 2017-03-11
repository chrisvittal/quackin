extern crate rusqlite;
extern crate csv;

pub mod data;
pub mod recommender;
pub mod sparse;
pub mod similarity;

/// Type alias for the user and item ids
pub type ID = usize;
