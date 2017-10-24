extern crate rstvnamer;
use std::path::PathBuf;

#[test]
fn test_parsing() {
    let path = PathBuf::from("scrubs.s01e12.avi");
    let f = rstvnamer::parse(&path).expect("Failed to parse");

    if let rstvnamer::ParsedFile::Season(x) = f {
        assert_eq!(x.series, "scrubs");
        assert_eq!(x.season, 1);
        assert_eq!(x.episode, 12);
    } else {
        panic!("Wrong parsed file type {:?}", f)
    }
}
