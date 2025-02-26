use std::path::Path;

const EXTENSIONS: [&str; 3] = ["jpg", "jpeg", "raf"];
pub fn is_valid_extension(path: &Path) -> bool {
    let extension = path.extension().unwrap_or("none".as_ref());
    let extension = extension.to_str().unwrap_or("none").to_lowercase();

    let res = !path.is_dir() && EXTENSIONS.contains(&extension.as_str());

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_truthy_for_jpg() {
        let path = Path::new("/example/path/my_image.jpg");
        assert!(is_valid_extension(path));

        let path = Path::new("/example/path/my_image.JPG");
        assert!(is_valid_extension(path));
    }

    #[test]
    fn it_should_be_truthy_for_jpeg() {
        let path = Path::new("/example/path/my_image.jpeg");
        assert!(is_valid_extension(path));

        let path = Path::new("/example/path/my_image.JPEG");
        assert!(is_valid_extension(path));
    }
}
