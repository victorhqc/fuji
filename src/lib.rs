//! # Fuji
//!
//! `fuji` is a library to read Fujifilm Recipes & Exif Metadata from a JPEG or RAF file using
//! [exiftool](https://exiftool.org/).

pub mod exif;
pub mod exiftool;
pub mod recipe;
mod utils;
