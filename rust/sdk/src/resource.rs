use std::collections::HashMap;
pub use futures::{Stream};
use crate::client::APIError;

pub struct Pagination {
    pub page: usize,
    pub items: usize
}
pub trait ByIdentifier {

}
pub trait Entity {

}
pub struct Identifier(pub HashMap<String, String>);

pub trait Paginated<T>
    where T: Entity {

    fn list(&self, settings: Pagination) -> Box<Stream<Item = T, Error = APIError> + Send>;
   // fn all(&self, batch_size: usize) -> Box<Stream<Item = T, Error = APIError> + Send>;
}

pub trait Retrieve<T>
    where T: Entity + ByIdentifier {

    fn get(&self, identifier: Identifier) -> Box<Stream<Item = T, Error = APIError> + Send>;

}