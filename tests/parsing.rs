extern crate rstvnamer;

#[test]
fn test_parsing(){
    let f = rstvnamer::parse("scrubs.s01e12").expect("Failed to parse");

    if let rstvnamer::parsing::ParsedFile::Season(x) = f {
        assert!(x.series == "scrubs");
        assert!(x.season == 1);
        assert!(x.episode == 12);
    } else {
        panic!("Wrong parsed file type {:?}", f)
    }
}
