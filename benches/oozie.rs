#![feature(test)]
extern crate test;
extern crate oozie;
extern crate csv;
extern crate rand;

use rand::{thread_rng, sample};
use oozie::data::*;
use oozie::similarity::*;
use oozie::recommender::*;
use csv::Reader;
use test::Bencher;


#[bench]
fn basic_get_user(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let mut rng = thread_rng();
    b.iter(|| {
        data_handler.get_user_ratings(*sample(&mut rng, &user_ids, 1)[0])
    });
}

#[bench]
fn sqlite_get_user(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let user_id = 100;
    let data_handler: SqliteDataHandler = SqliteDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let mut rng = thread_rng();
    b.iter(|| {
        data_handler.get_user_ratings(*sample(&mut rng, &user_ids, 1)[0])
    });
}