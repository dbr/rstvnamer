extern crate rstvnamer;

#[test]
fn test_parsing(){
    let f = rstvnamer::parse("scrubs.s01e12");
    assert!(f.is_some());

    match f.unwrap() {
        rstvnamer::ParsedFile::Season(x) => assert!(x.series == "scrubs"),
        _ => assert!(false),
    }
}
