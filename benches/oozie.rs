#![feature(test)]
extern crate test;
extern crate oozie;
extern crate csv;
extern crate rand;

use rand::{thread_rng, sample};
use oozie::data::*;
use oozie::recommender::*;
use oozie::similarity::*;
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
fn basic_get_item(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let item_ids = data_handler.get_item_ids();
    let mut rng = thread_rng();
    b.iter(|| {
        data_handler.get_item_ratings(*sample(&mut rng, &item_ids, 1)[0])
    });
}

#[bench]
fn basic_user_predict(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let item_ids = data_handler.get_item_ids();
    let recommender: BasicUserRecommender<BasicDataHandler> =
        BasicUserRecommender::new(&mut data_handler, 0.0, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        let item_id = *sample(&mut rng, &item_ids, 1)[0];
        recommender.predict(user_id, item_id)
    });
}

#[bench]
fn knn_user_predict(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let item_ids = data_handler.get_item_ids();
    let recommender: KNNUserRecommender<BasicDataHandler> =
        KNNUserRecommender::new(&mut data_handler, 20, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        let item_id = *sample(&mut rng, &item_ids, 1)[0];
        recommender.predict(user_id, item_id)
    });
}

#[bench]
fn basic_item_predict(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let item_ids = data_handler.get_item_ids();
    let recommender: BasicItemRecommender<BasicDataHandler> =
        BasicItemRecommender::new(&mut data_handler, 0.0, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        let item_id = *sample(&mut rng, &item_ids, 1)[0];
        recommender.predict(user_id, item_id)
    });
}

#[bench]
fn knn_item_predict(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let item_ids = data_handler.get_item_ids();
    let recommender: KNNItemRecommender<BasicDataHandler> =
        KNNItemRecommender::new(&mut data_handler, 20, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        let item_id = *sample(&mut rng, &item_ids, 1)[0];
        recommender.predict(user_id, item_id)
    });
}

#[bench]
#[ignore]
fn basic_user_recommend(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let recommender: BasicUserRecommender<BasicDataHandler> =
        BasicUserRecommender::new(&mut data_handler, 0.0, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        recommender.recommend(user_id)
    });
}

#[bench]
#[ignore]
fn basic_item_recommend(b: &mut Bencher) {
    let reader = Reader::from_file("./data/movielens.csv")
        .unwrap().delimiter(b' ');
    let mut data_handler: BasicDataHandler = BasicDataHandler::from_reader(reader);
    let user_ids = data_handler.get_user_ids();
    let recommender: BasicItemRecommender<BasicDataHandler> =
        BasicItemRecommender::new(&mut data_handler, 0.0, cosine);
    let mut rng = thread_rng();
    b.iter(|| {
        let user_id = *sample(&mut rng, &user_ids, 1)[0];
        recommender.recommend(user_id)
    });
}
