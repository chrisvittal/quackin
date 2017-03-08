extern crate oozie;
extern crate csv;

use oozie::data::{BasicDataHandler, DataHandler};
use oozie::similarity::cosine;
use oozie::recommender::NearestUsersRecommender;
use csv::Reader;

#[test]
fn it_works() {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    println!("{:?}", data_handler.get_item_ids());
}
