#[test]
fn test_parsing(){
    let f = super::parse().unwrap();
    assert!(f.filename == "a.avi");
}
