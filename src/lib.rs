//! # Quackin - API documentation
//!
//! Hi, welcome to the Quackin API documentation!
//!
//! Quackin is a recommender systems framework written in Rust focused on:
//!
//! - Facilitate data handling.
//! - Providing collaborative filtering algorithms.
//! - Being an environment to build, test and evaluate new algorithms.
//!
//! Until we get a website for the crate, this docs will be the primary
//! reference for both technical and user-side aspects of Quackin.
//!
//! Quackin was called Oozie as a reference to Apache Mahout, but then I
//! discovered that Apache Oozie already exists. Now is called Quackin because
//! I wanted a funny name and my debugger rubber duck was on my desk at that
//! moment.
//!
//! ## Usage
//! To start using Quackin just add it as a dependency to your `Cargo.toml` file:
//!
//! ```ignore
//! [dependencies]
//! quackin = "0.1.1"
//! ```
//!
//! ## Getting started
//! We will write a simple movie recommender engine using Quackin over one of
//! the movielens dataset (download `ml-latest-small.zip` from
//! [here](https://grouplens.org/datasets/movielens/)).
//!
//! ### Loading data into Quackin
//! First we need to load the movielens dataset into Rust. This dataset has the
//! following columns: `userId`, `movieId`, `rating` and `timestamp`. We need
//! to write a `struct` to store this records
//!
//! ```ignore
//! struct MyRecord {
//!     user_id: u32,
//!     movie_id: u32,
//!     rating: f64,
//!     timestamp: u32
//! }
//! ```
//! The order of the fields must be the same as the order of the columns in the
//! dataset, but the name of them is not important, we could use any name we
//! like. Now we need to implement the `Record` trait for `MyRecord`
//!
//! ```ignore
//! use quackin::data::{Record, read_records};
//!
//! impl Record<u32, u32> for MyRecord {
//!     fn get_user_id(&self) -> &u32 {
//!         &self.user_id
//!     }
//!     fn get_item_id(&self) -> &u32 {
//!         &self.movie_id
//!     }
//!     fn get_rating(&self) -> f64 {
//!         self.rating
//!     }
//! }
//! ```
//! Whew, thats a lot of boilerplate for just loading the dataset. But this was
//! the hardest part of the process. Now lets load the dataset:
//!
//! ```ignore
//! let records: Vec<MyRecord> = read_records("/path/to/movielens", None, true);
//! ```
//! Thats it! now let's build a recommender
//!
//! ### Creating a recommender
//! We will use a traditional K-nearest neighbors algorithm to build a basic
//! recommender. For each user we will take the 50 nearest users to him,
//! using a cosine similarity
//!
//! ```ignore
//! use quackin::recommender::KnnUserRecommender:
//! use quackin::metrics::similarity::cosine;
//!
//! let recommender = KnnUserRecommender::from_records(&records, cosine, 50);
//! ```
//! Now we can ask predictions from the recommender. For example, the user with
//! user ID `1` gave a rating of 4 to the classic science fiction movie "Tron".
//! What would our recommender predict about this?
//!
//! ```ignore
//! println!("{:?}", recommender.predict(&1, &2105));
//! // Ok(3.504942020280084)
//! ```
//! Not bad! we aren't that far from the real rating. Now you can start recommending
//! movies to everyone!
extern crate csv;
extern crate sprs;
extern crate rustc_serialize;

pub mod data;
pub mod recommender;
pub mod metrics;
