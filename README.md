# Quackin

[![travis](https://img.shields.io/travis/z1mvader/quackin.svg)](https://travis-ci.org/z1mvader/quackin)  [![crates](https://img.shields.io/crates/v/quackin.svg)](https://crates.io/crates/quackin)

Release the quackin! 🦆

Quackin is a recommender systems framework written in Rust. This is a young
project, which means two things:

- There will be a lot of breaking changes until version 1.0
- Is a perfect project for your contributions ;)

Quackin was concieved as a framework for collaborative filtering recommender
systems. But it is open to new ideas. If you want to contribute in some way
open or start working on an issue or ping me at z1mvader@protonmail.com.

One of the difficulties for developing this crate is that the Rust's machine
learning and data mining [ecosystem](http://www.arewelearningyet.com/) is on
an early stage, then there is no standard libraries for anything yet.
Nevertheless, I believe that we should take advantage of the control and speed
that Rust offers to provide a competitive alternative to frameworks written
in languages with a more developed ecosystem like Python or Java.

## Getting started
We will write a simple movie recommender engine using Quackin over one of
the movielens dataset. Download `ml-latest-small.zip` from
[here](https://grouplens.org/datasets/movielens/), and then extract the
`ratings.csv` file.

### Loading data into Quackin
First we need to load the movielens dataset into Rust. This dataset has the
following columns: `userId`, `movieId`, `rating` and `timestamp`. We must
tell quackin that this file has this columns in this specific order. Also,
we need to specify if this file has headers or not and which delimiter is
separating the values`.

```rust
use quackin::data::{Record, ReadOptions, read_custom_records};
quackin::data::Field::*;

let options = ReadOptions::custom(vec![UserID, ItemID, Rating, Other], true, ',').unwrap();
//                                                             ^^^^^   ^^^^  ^^^
//                                                             |       |     |
//                             we don't care about the timestamp.      |     |
//                                                 this file has headers.    |
//                                                    use comma as a delimiter.
```
Whew, thats a lot of boilerplate for just loading the dataset. But this was
the hardest part of the process. Now lets load the dataset:

```rust
let records = read_custom_records("/path/to/movielens", options);
```
Thats it! now let's build a recommender

### Creating a recommender
We will use a traditional K-nearest neighbors algorithm to build a basic
recommender. For each user we will take the 50 nearest users to him,
using a cosine similarity

```rust
use quackin::recommender::KnnUserRecommender;
use quackin::metrics::similarity::cosine;

let recommender = KnnUserRecommender::from_records(&records, cosine, 50);
```
Now we can ask predictions from the recommender. For example, the user with
user ID `1` gave a rating of 4 to the classic science fiction movie "Tron".
What would our recommender predict about this?

```rust
println!("{:?}", recommender.predict("1", "2105"));
// Ok(3.504942020280084)
```
Not bad! we aren't that far from the real rating. Now you can start recommending
movies to everyone!
