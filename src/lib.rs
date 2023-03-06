pub use crate::csv::get;
pub use crate::model::data::*;

mod download;
mod csv;
mod error;
mod model;
#[cfg(test)]
mod tests;
mod util;

