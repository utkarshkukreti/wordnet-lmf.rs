#[test]
fn t() {
    let file = wordnet_lmf::File::open("tests/fixtures/english-wordnet-2020.xml").unwrap();

    assert_eq!(file.lexicons.len(), 1);

    let lexicon = &file.lexicons[0];

    assert_eq!(lexicon.id, "ewn");
}
