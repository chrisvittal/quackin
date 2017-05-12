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
the movielens dataset (download `ml-latest-small.zip` from
[here](https://grouplens.org/datasets/movielens/)).

### Loading data into Quackin
First we need to load the movielens dataset into Rust. This dataset has the
following columns: `userId`, `movieId`, `rating` and `timestamp`. We need
to write a `struct` to store this records

```rust
struct MyRecord {
    user_id: u32,
    movie_id: u32,
    rating: f64,
    timestamp: u32
}
```
The order of the fields must be the same as the order of the columns in the
dataset, but the name of them is not important, we could use any name we
like. Now we need to implement the `Record` trait for `MyRecord`

```rust
use quackin::data::{Record, read_records};

impl Record<u32, u32> for MyRecord {
    fn get_user_id(&self) -> &u32 {
        &self.user_id
    }
    fn get_item_id(&self) -> &u32 {
        &self.movie_id
    }
    fn get_rating(&self) -> f64 {
        self.rating
    }
}
```
Whew, thats a lot of boilerplate for just loading the dataset. But this was
the hardest part of the process. Now lets load the dataset:

```rust
let records: Vec<MyRecord> = read_records("/path/to/movielens", None, true);
```
Thats it! now let's build a recommender

### Creating a recommender
We will use a traditional K-nearest neighbors algorithm to build a basic
recommender. For each user we will take the 50 nearest users to him,
using a cosine similarity

```rust
use quackin::recommender::KnnUserRecommender:

let recommender = KnnUserRecommender::from_records(&records, cosine, 50);
```
Now we can ask predictions from the recommender. For example, the user with
user ID `1` gave a rating of 4 to the classic science fiction movie "Tron".
What would our recommender predict about this?

```rust
println!("{:?}", recommender.predict(&1, &2105));
// Ok(3.504942020280084)
```
Not bad! we aren't that far from the real rating. Now you can start recommending
movies to everyone!
