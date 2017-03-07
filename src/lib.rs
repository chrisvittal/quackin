extern crate csv;
extern crate rustc_serialize;

use std::hash::Hash;
use rustc_serialize::Decodable;

pub mod data;
pub mod recommender;
pub mod sparse;
pub mod similarity;

/// Basic trait alias that represents what a type should have to be used as a key in a HashMap
/// If you have a better workaround for this please write me!
pub trait Key: Eq + Hash + Decodable + Clone {}
impl<T> Key for T where T: Eq + Hash + Decodable + Clone {}

