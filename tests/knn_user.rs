extern crate csv;
extern crate sprs;
extern crate oozie;

use sprs::{CsVecOwned};
use oozie::recommender::collaborative::KnnUserRecommender;
use oozie::data::{BasicDataSet, DataSet};
use oozie::measure::similarity::cosine;
use csv::Reader;

#[test]
fn it_works() {
    let user_reader = Reader::from_file("data/mini_set/users.csv").unwrap();
    let item_reader = Reader::from_file("data/mini_set/items.csv").unwrap();
    let rating_reader = Reader::from_file("data/mini_set/ratings.csv").unwrap();

    let data_set = BasicDataSet::from_csv(user_reader, item_reader, rating_reader);
    let recommender = KnnUserRecommender::from_dataset(data_set, cosine);

    println!("{}", recommender.predict("user_1", "item_0"));
    println!("{}", recommender.predict("user_1", "item_1"));
    println!("{}", recommender.predict("user_1", "item_2"));
    println!("{:?}", recommender.recommend("user_1"));
}
