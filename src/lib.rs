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
//! quackin = "*"
//! ```
//!

extern crate csv;
extern crate sprs;
extern crate rustc_serialize;

pub mod data;
pub mod recommender;
pub mod metrics;
