extern crate oozie;
extern crate csv;

use oozie::data::{BasicDataHandler, DataHandler};
use csv::Reader;

#[test]
fn it_works() {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let data_handler: BasicDataHandler<usize, usize> = BasicDataHandler::from_reader(reader);
    println!("{:?}", data_handler.get_user_ratings(326));
}
