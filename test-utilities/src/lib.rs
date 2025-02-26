use std::path::PathBuf;

pub fn get_manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR")
        .into()
}
