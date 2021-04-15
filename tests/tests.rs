use expect_test::{expect, Expect};

#[test]
fn t() {
    let file = wordnet_lmf::File::open("tests/fixtures/english-wordnet-2020.xml").unwrap();

    assert_eq!(file.lexicons.len(), 1);

    let lexicon = &file.lexicons[0];

    assert_eq!(lexicon.id, "ewn");
    assert_eq!(lexicon.label, "English WordNet");
    assert_eq!(lexicon.language, "en");
    assert_eq!(lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!(lexicon.version, "2020");
    assert_eq!(
        lexicon.url,
        "https://github.com/globalwordnet/english-wordnet"
    );

    assert_eq!(lexicon.lexical_entries.len(), 163079);

    check(
        &lexicon.lexical_entries[..10],
        expect![[r#"
        [
            LexicalEntry {
                id: "ewn-occultism-n",
            },
            LexicalEntry {
                id: "ewn-probability_theory-n",
            },
            LexicalEntry {
                id: "ewn-dermatology-n",
            },
            LexicalEntry {
                id: "ewn-omphaloskepsis-n",
            },
            LexicalEntry {
                id: "ewn-rote-n",
            },
            LexicalEntry {
                id: "ewn-pilot-n",
            },
            LexicalEntry {
                id: "ewn-symbolization-n",
            },
            LexicalEntry {
                id: "ewn-orthodontics-n",
            },
            LexicalEntry {
                id: "ewn-peace_advocacy-n",
            },
            LexicalEntry {
                id: "ewn-culture-n",
            },
        ]
    "#]],
    );
}

fn check<T: std::fmt::Debug>(t: T, expect: Expect) {
    expect.assert_debug_eq(&t)
}
