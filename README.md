# Fuji

Read Fujifilm Recipe Settings from EXIF using [exiftool](https://exiftool.org/).

_Note: This is not production ready and not recommended for everyday use until
it can be shipped without the need for exiftool. However it works quite well
with its limitations._

### Why Exiftool?

I tried using [kamadak-exif](https://crates.io/crates/kamadak-exif) but it does
not read the EXIF data that I was interested in. There's also
[rexiv2](https://crates.io/crates/rexiv2) that seems to read the values needed
for Fujifilm recipes, a later revision of this library will probably use this
instead or as a different feature branch.

[Exiftool](https://exiftool.org/) on the other hand is quite powerful, it has a
lot of support for and it works wonderfully. The only downside is the need of
spawning a separate process to invoke it with Perl or run the executable in
Windows.

This makes it awkward to ship, as it requires Perl to be installed on the
system. As well as shipping the exiftool executable.

## Example

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
