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
                lemma: Lemma {
                    written_form: "occultism",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-occultism-n-05977317-01",
                        synset: "ewn-05977317-n",
                    },
                    Sense {
                        id: "ewn-occultism-n-05977155-01",
                        synset: "ewn-05977155-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-probability_theory-n",
                lemma: Lemma {
                    written_form: "probability theory",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-probability_theory-n-06046620-01",
                        synset: "ewn-06046620-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-dermatology-n",
                lemma: Lemma {
                    written_form: "dermatology",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-dermatology-n-06059031-01",
                        synset: "ewn-06059031-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-omphaloskepsis-n",
                lemma: Lemma {
                    written_form: "omphaloskepsis",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-omphaloskepsis-n-05795853-01",
                        synset: "ewn-05795853-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-rote-n",
                lemma: Lemma {
                    written_form: "rote",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-rote-n-05763390-01",
                        synset: "ewn-05763390-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-pilot-n",
                lemma: Lemma {
                    written_form: "pilot",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-pilot-n-10452928-01",
                        synset: "ewn-10452928-n",
                    },
                    Sense {
                        id: "ewn-pilot-n-10453216-01",
                        synset: "ewn-10453216-n",
                    },
                    Sense {
                        id: "ewn-pilot-n-06633413-03",
                        synset: "ewn-06633413-n",
                    },
                    Sense {
                        id: "ewn-pilot-n-05947118-03",
                        synset: "ewn-05947118-n",
                    },
                    Sense {
                        id: "ewn-pilot-n-03945557-03",
                        synset: "ewn-03945557-n",
                    },
                    Sense {
                        id: "ewn-pilot-n-03333146-04",
                        synset: "ewn-03333146-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-symbolization-n",
                lemma: Lemma {
                    written_form: "symbolization",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-symbolization-n-06614677-01",
                        synset: "ewn-06614677-n",
                    },
                    Sense {
                        id: "ewn-symbolization-n-05773412-02",
                        synset: "ewn-05773412-n",
                    },
                    Sense {
                        id: "ewn-symbolization-n-00413284-02",
                        synset: "ewn-00413284-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-orthodontics-n",
                lemma: Lemma {
                    written_form: "orthodontics",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-orthodontics-n-06058083-01",
                        synset: "ewn-06058083-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-peace_advocacy-n",
                lemma: Lemma {
                    written_form: "peace advocacy",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-peace_advocacy-n-06231604-01",
                        synset: "ewn-06231604-n",
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-culture-n",
                lemma: Lemma {
                    written_form: "culture",
                    part_of_speech: "n",
                },
                senses: [
                    Sense {
                        id: "ewn-culture-n-08304765-01",
                        synset: "ewn-08304765-n",
                    },
                    Sense {
                        id: "ewn-culture-n-05759791-01",
                        synset: "ewn-05759791-n",
                    },
                    Sense {
                        id: "ewn-culture-n-05993821-02",
                        synset: "ewn-05993821-n",
                    },
                    Sense {
                        id: "ewn-culture-n-00922324-01",
                        synset: "ewn-00922324-n",
                    },
                    Sense {
                        id: "ewn-culture-n-14483408-03",
                        synset: "ewn-14483408-n",
                    },
                    Sense {
                        id: "ewn-culture-n-06204144-01",
                        synset: "ewn-06204144-n",
                    },
                    Sense {
                        id: "ewn-culture-n-00919573-01",
                        synset: "ewn-00919573-n",
                    },
                ],
            },
        ]
    "#]],
    );
}

fn check<T: std::fmt::Debug>(t: T, expect: Expect) {
    expect.assert_debug_eq(&t)
}
