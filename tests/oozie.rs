extern crate oozie;
extern crate csv;

use oozie::data::{BasicDataHandler, DataHandler};
use oozie::similarity::cosine;
use oozie::recommender::{Recommender, NearestUserRecommender};
use csv::Reader;

#[test]
fn it_works() {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let user_id = 100;
    let data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let recommender: NearestUserRecommender<BasicDataHandler> =
        NearestUserRecommender::new(&data_handler, cosine, 50);
    let user = data_handler.get_user_ratings(user_id);
    let mut recommendations = recommender.recommend(user_id);
    recommendations.truncate(100);
    println!("{:?}", user);
    println!("{:?}", recommendations);
}
