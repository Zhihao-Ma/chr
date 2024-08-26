#[macro_use]
extern crate getset;
#[macro_use]
extern crate rbatis;
pub mod config;
pub mod controller;
pub mod error;
pub mod mapper;
pub mod model;
pub mod service;
pub mod utils;

pub static APPLICATION_CONTEXT: state::Container!(Send + Sync) =
    <state::Container![Send + Sync]>::new();

