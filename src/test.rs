#[test]
fn test_parsing(){
    let f = super::parse("scrubs.s01e12");
    assert!(f.is_some());

    match f.unwrap() {
        super::ParsedFile::Season(x) => assert!(x.series == "scrubs"),
        _ => assert!(false),
    }
}
