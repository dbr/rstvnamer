extern crate rstvnamer;
use std::path::PathBuf;

#[test]
fn test_good() {
    let path = PathBuf::from("scrubs.s01e12.avi");
    let f = rstvnamer::parse(&path).expect("Failed to parse");
    let p = rstvnamer::populate(&f).expect("Failed to populate");
    assert_eq!(p.series, "Scrubs");
    assert_eq!(p.season, 1);
    assert_eq!(p.episode, 12);
    assert_eq!(p.episodename, "My Blind Date");
}

#[test]
fn test_not_found() {
    let path = PathBuf::from("scrubs.s01e33.avi");
    let f = rstvnamer::parse(&path).expect("Failed to parse");
    assert!(rstvnamer::populate(&f).is_err(), "episode should not exist");
}
