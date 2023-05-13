#![warn(clippy::all, rust_2018_idioms)]

#[macro_use] extern crate serde;

mod app;
pub use app::App;

mod replay;
