#[macro_use] extern crate serde;
extern crate loriot_api;
extern crate reqwest;
extern crate futures;
extern crate futures_locks;

pub mod resource;
pub mod client;
pub mod authentication;
pub mod modules;
#[cfg(test)]
extern crate tokio;