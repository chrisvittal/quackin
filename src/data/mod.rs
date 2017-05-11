use csv;
use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
pub struct BaseRecord<U, I> {
    user_id: U,
    item_id: I,
    rating: f64,
}

pub type DefaultRecord = BaseRecord<String, String>;

pub fn read_ratings<R>(path: &str, delimiter: Option<char>, has_headers: bool) -> Result<Vec<R>, csv::Error> where R: Decodable {
    let del = match delimiter {
        Some(del) => del as u8,
        None => ',' as u8
    };

    let mut reader = try!(csv::Reader::from_file(path)).has_headers(has_headers).delimiter(del);
    let ratings = reader.decode().map(|record| {record.unwrap()}).collect::<Vec<R>>();

    Ok(ratings)
}
