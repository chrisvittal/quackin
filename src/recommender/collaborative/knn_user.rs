use std::collections::HashMap;
use sprs::CsVecOwned;
use measure::similarity::Similarity;
use data::DataSet;

pub struct KnnUserRecommender {
    user_vectors: HashMap<String, CsVecOwned<f64>>,
    item_indices: HashMap<String, usize>,
    item_ids: HashMap<usize, String>,
    similarity: Similarity
}

impl KnnUserRecommender {
     pub fn from_dataset<D: DataSet>(data_set: D, similarity: Similarity) -> Self {
         let user_vectors = data_set.get_user_vectors();
         let item_indices = data_set.get_item_indices();
         let item_ids: HashMap<usize, String> = item_indices.clone().into_iter().map(|(k, v)| (v, k)).collect();
         Self {
             user_vectors: user_vectors,
             item_indices: item_indices,
             item_ids: item_ids,
             similarity: similarity
         }
    }

    pub fn predict(&self, user_id: &str, item_id: &str) -> f64 {
        let item_index = self.item_indices.get(item_id).unwrap();
        let user_vector = self.user_vectors.get(user_id).unwrap();

        let mut neigh_ratings = self.user_vectors.iter()
            .filter_map(|(_, other_vector)| {
                match other_vector.get(*item_index) {
                    Some(rating) => Some((*rating, (self.similarity)(&user_vector, other_vector))),
                    None => None
                }
            }).collect::<Vec<(f64, f64)>>();

        neigh_ratings.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
        neigh_ratings.truncate(50);

        let mut pred_rating = 0.0;
        let mut total_sim = 0.0;
        for (r, s) in neigh_ratings {
            pred_rating += r*s;
            total_sim += s;
        }
        if total_sim > 0.0 {
            pred_rating /= total_sim;
        }
        pred_rating
    }

    pub fn recommend(&self, user_id: &str) -> Vec<(String, f64)> {
        let user_vector = self.user_vectors.get(user_id).unwrap();

        let mut neigh_vectors = self.user_vectors.iter()
            .map(|(_, other_vector)| {
                let s = (self.similarity)(&user_vector, other_vector);
                (other_vector.to_owned(), s)
            }).collect::<Vec<(CsVecOwned<f64>, f64)>>();

        neigh_vectors.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
        neigh_vectors.truncate(50);

        let mut pred_ratings = CsVecOwned::<f64>::empty(self.item_indices.len());
        let mut tot_sim = CsVecOwned::<f64>::empty(self.item_indices.len());
        for (v, s) in neigh_vectors {
            pred_ratings = &pred_ratings + &v.map(|x| s*x);
            tot_sim = &tot_sim + &v.map(|_| s);
        }

        let mut recommendation = Vec::new();
        for ((item_index_1, &r), (item_index_2, &s)) in pred_ratings.iter().zip(tot_sim.iter()) {
            if item_index_1 == item_index_2 {
                let item_id = self.item_ids.get(&item_index_1).unwrap();
                if s > 0.0 {
                    recommendation.push((item_id.clone(), r/s));
                }
                else {
                    recommendation.push((item_id.clone(), 0.0));
                }
            }
        }
        recommendation.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
        recommendation
    }
}
