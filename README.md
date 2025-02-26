# Fuji
Read Fujifilm Recipe Settings from EXIF using [exiftool](https://exiftool.org/).

```rust
use fuji::exiftool::spawn;
use fuji::recipe::read;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read metadata from a Fujifilm image
    let path = std::path::Path::new("tests/img/DSCF5230.JPG");
    let metadata = spawn::read_metadata(&path, None)?;

    // Parse the Fujifilm recipe from the metadata
    let recipe = read::from_exif(&metadata)?;

    // Recipe will contain details like FilmSimulation, WhiteBalance, etc.
    if let Some(details) = recipe {
        println!("Film Simulation: {:?}", details.film_simulation);
        println!("White Balance: {:?}", details.settings);
    }
    Ok(())
}
```

## Requirements

- Perl (To Run Exiftool, MacOSX, Linux Only, Windows not needed)
- Git (To Run Exiftool install script)
- Rust

## Installation

The library needs to have Perl available in the PATH Environment, or the
Exiftool executable for Windows.

```sh
./scripts/unix/exiftool.sh
```

```bat
.\scripts\windows\exiftool.bat
```

## Usage

Check the `tests` directory for examples of usage.
