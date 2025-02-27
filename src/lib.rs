//! # Fuji
//!
//! `fuji` is a library to read Fujifilm Recipes & Exif Metadata from a JPEG or RAF file using
//! [exiftool](https://exiftool.org/).
//!
//! ## Example
//!
//! ```
//! use fuji::exiftool::spawn;
//! use fuji::recipe::read;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Read metadata from a Fujifilm image
//! let path = std::path::Path::new("tests/img/DSCF5230.JPG");
//! let metadata = spawn::read_metadata(&path, None)?;
//!
//! // Parse the Fujifilm recipe from the metadata
//! let recipe = read::from_exif(&metadata)?;
//!
//! // Recipe will contain details like FilmSimulation, WhiteBalance, etc.
//! if let Some(details) = recipe {
//!     println!("Film Simulation: {:?}", details.film_simulation);
//!     println!("White Balance: {:?}", details.settings);
//! }
//! # Ok(())
//! # }

pub mod exif;
pub mod exiftool;
pub mod recipe;
mod utils;
