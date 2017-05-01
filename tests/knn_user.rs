extern crate sprs;
extern crate oozie;

use sprs::{CsVecOwned};
use oozie::recommender::collaborative::KnnUserRecommender;
use oozie::measure::similarity::cosine;

#[test]
fn add_user_test() {
    let expected = CsVecOwned::<f64>::empty(0);
    let mut recommender = KnnUserRecommender::empty(cosine);
    recommender.add_user("new_user").unwrap();
    assert_eq!(expected, recommender.get_user_vector("new_user").unwrap());
}

#[test]
fn add_item_test() {
    let expected = CsVecOwned::<f64>::empty(1);
    let mut recommender = KnnUserRecommender::empty(cosine);
    recommender.add_user("user_0").unwrap();
    recommender.add_item("item_0").unwrap();
    assert_eq!(expected, recommender.get_user_vector("user_0").unwrap());
}

#[test]
fn get_inexistent_user_vector_test() {
    let mut recommender = KnnUserRecommender::empty(cosine);
    recommender.add_user("new_user").unwrap();
    recommender.get_user_vector("not_new_user").unwrap_err();
}

#[test]
fn get_inexistent_item_index_test() {
    let mut recommender = KnnUserRecommender::empty(cosine);
    recommender.add_item("new_item").unwrap();
    recommender.get_item_index("not_new_item").unwrap_err();
}
