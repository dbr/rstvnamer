extern crate rstvnamer;
use std::path::PathBuf;

#[test]
fn test_parsing() {
    let path = PathBuf::from("scrubs.s01e12.avi");
    let f = rstvnamer::parse(&path).expect("Failed to parse");

    if let rstvnamer::ParsedFile::Season(x) = f {
        assert!(x.series == "scrubs");
        assert!(x.season == 1);
        assert!(x.episode == 12);
    } else {
        panic!("Wrong parsed file type {:?}", f)
    }
}
