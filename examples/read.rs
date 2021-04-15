fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("must pass lmf file path as argument");
    let timer = std::time::Instant::now();
    let file = wordnet_lmf::File::open(path).unwrap();
    dbg!(timer.elapsed());
    dbg!(file.lexicons.len());
    for lexicon in &*file.lexicons {
        dbg!(
            &lexicon.id,
            &lexicon.label,
            &lexicon.language,
            lexicon.lexical_entries.len(),
            lexicon.synsets.len()
        );
    }
}
