use expect_test::{expect, Expect};

#[test]
fn t() {
    let file = wordnet_lmf::File::open("tests/fixtures/english-wordnet-2020.xml").unwrap();

    assert_eq!(file.lexicons.len(), 1);

    let lexicon = &file.lexicons[0];

    assert_eq!(&*lexicon.id, "ewn");
    assert_eq!(&*lexicon.label, "English WordNet");
    assert_eq!(&*lexicon.language, "en");
    assert_eq!(&*lexicon.email, "english-wordnet@googlegroups.com");
    assert_eq!(
        &*lexicon.license,
        "https://creativecommons.org/licenses/by/4.0/"
    );
    assert_eq!(&*lexicon.version, "2020");
    assert_eq!(
        &*lexicon.url,
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
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-occultism-n-05977317-01",
                        synset: "ewn-05977317-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-occultist-n-10390080-01",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-occultism-n-05977155-01",
                        synset: "ewn-05977155-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-occultist-n-10390080-01",
                            },
                        ],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-probability_theory-n",
                lemma: Lemma {
                    written_form: "probability theory",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-probability_theory-n-06046620-01",
                        synset: "ewn-06046620-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-dermatology-n",
                lemma: Lemma {
                    written_form: "dermatology",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-dermatology-n-06059031-01",
                        synset: "ewn-06059031-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-dermatologic-a-02927543-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-dermatological-a-02927543-02",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-dermatologist-n-10025839-01",
                            },
                        ],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-omphaloskepsis-n",
                lemma: Lemma {
                    written_form: "omphaloskepsis",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-omphaloskepsis-n-05795853-01",
                        synset: "ewn-05795853-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-rote-n",
                lemma: Lemma {
                    written_form: "rote",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-rote-n-05763390-01",
                        synset: "ewn-05763390-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-pilot-n",
                lemma: Lemma {
                    written_form: "pilot",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-pilot-n-10452928-01",
                        synset: "ewn-10452928-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-pilot-v-01944952-03",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-pilot-n-10453216-01",
                        synset: "ewn-10453216-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-pilot-v-01937276-02",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-pilot-n-06633413-03",
                        synset: "ewn-06633413-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-pilot-n-05947118-03",
                        synset: "ewn-05947118-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-pilot-n-03945557-03",
                        synset: "ewn-03945557-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-pilot-n-03333146-04",
                        synset: "ewn-03333146-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-symbolization-n",
                lemma: Lemma {
                    written_form: "symbolization",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-symbolization-n-06614677-01",
                        synset: "ewn-06614677-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-symbolize-v-00989629-01",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-symbolization-n-05773412-02",
                        synset: "ewn-05773412-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-symbolize-v-00837915-02",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-symbolization-n-00413284-02",
                        synset: "ewn-00413284-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-orthodontics-n",
                lemma: Lemma {
                    written_form: "orthodontics",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-orthodontics-n-06058083-01",
                        synset: "ewn-06058083-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-orthodontic-a-02927128-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-orthodontist-n-10404309-01",
                            },
                        ],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-peace_advocacy-n",
                lemma: Lemma {
                    written_form: "peace advocacy",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-peace_advocacy-n-06231604-01",
                        synset: "ewn-06231604-n",
                        relations: [],
                    },
                ],
            },
            LexicalEntry {
                id: "ewn-culture-n",
                lemma: Lemma {
                    written_form: "culture",
                    part_of_speech: "n",
                },
                forms: [],
                senses: [
                    Sense {
                        id: "ewn-culture-n-08304765-01",
                        synset: "ewn-08304765-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-acculturate-v-00159688-01",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-culture-n-05759791-01",
                        synset: "ewn-05759791-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-cultural-s-02256437-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-cultural-a-02910174-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-acculturate-v-00159688-01",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-culture-n-05993821-02",
                        synset: "ewn-05993821-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-cultural-s-02256437-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-cultural-a-02883805-01",
                            },
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-acculturate-v-00159688-01",
                            },
                        ],
                    },
                    Sense {
                        id: "ewn-culture-n-00922324-01",
                        synset: "ewn-00922324-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-culture-n-14483408-03",
                        synset: "ewn-14483408-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-culture-n-06204144-01",
                        synset: "ewn-06204144-n",
                        relations: [],
                    },
                    Sense {
                        id: "ewn-culture-n-00919573-01",
                        synset: "ewn-00919573-n",
                        relations: [
                            SenseRelation {
                                rel_type: "derivation",
                                target: "ewn-cultural-a-02883666-01",
                            },
                        ],
                    },
                ],
            },
        ]
        "#]],
    );

    check(
        &lexicon
            .lexical_entries
            .iter()
            .filter(|lexical_entry| ["genus"].contains(&&*lexical_entry.lemma.written_form))
            .collect::<Vec<_>>(),
        expect![[r#"
            [
                LexicalEntry {
                    id: "ewn-genus-n",
                    lemma: Lemma {
                        written_form: "genus",
                        part_of_speech: "n",
                    },
                    forms: [
                        Form {
                            written_form: "genera",
                        },
                        Form {
                            written_form: "genus",
                        },
                    ],
                    senses: [
                        Sense {
                            id: "ewn-genus-n-05853540-01",
                            synset: "ewn-05853540-n",
                            relations: [
                                SenseRelation {
                                    rel_type: "derivation",
                                    target: "ewn-generic-s-01105857-01",
                                },
                            ],
                        },
                        Sense {
                            id: "ewn-genus-n-08125938-01",
                            synset: "ewn-08125938-n",
                            relations: [
                                SenseRelation {
                                    rel_type: "derivation",
                                    target: "ewn-generic-a-02744752-01",
                                },
                            ],
                        },
                    ],
                },
            ]
        "#]],
    );

    assert_eq!(lexicon.synsets.len(), 120053);

    check(
        &lexicon.synsets[..10],
        expect![[r#"
        [
            Synset {
                id: "ewn-05619057-n",
                part_of_speech: "n",
                definitions: [
                    "that which is responsible for one\'s thoughts, feelings, and conscious brain functions; the seat of the faculty of reason",
                ],
                examples: [
                    "\"his mind wandered\"",
                    "\"I couldn\'t get his words out of my head\"",
                ],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-00023451-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05619467-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620826-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620953-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05621057-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05621261-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05619467-n",
                part_of_speech: "n",
                definitions: [
                    "an informal British expression for head or mind",
                ],
                examples: [
                    "\"use your noddle\"",
                ],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05619057-n",
                    },
                    SynsetRelation {
                        rel_type: "domain_region",
                        target: "ewn-08879115-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05619605-n",
                part_of_speech: "n",
                definitions: [
                    "an abstract mental location",
                ],
                examples: [
                    "\"he has a special place in my thoughts\"",
                    "\"a place in my heart\"",
                    "\"a political system with no place for the less prominent groups\"",
                ],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-00023451-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-06257026-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05619850-n",
                part_of_speech: "n",
                definitions: [
                    "knowledge that is available to anyone",
                ],
                examples: [],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-00023451-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620024-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620489-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620592-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620024-n",
                part_of_speech: "n",
                definitions: [
                    "anything generally known to everyone",
                ],
                examples: [],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05619850-n",
                    },
                    SynsetRelation {
                        rel_type: "hyponym",
                        target: "ewn-05620314-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620141-n",
                part_of_speech: "n",
                definitions: [
                    "the body of ideas that determine the knowledge that is intellectually certain at any particular time",
                ],
                examples: [],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-00023451-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620314-n",
                part_of_speech: "n",
                definitions: [
                    "knowledge of some recent fact or event that has become so commonly known that it has lost its original pertinence",
                ],
                examples: [],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05620024-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620489-n",
                part_of_speech: "n",
                definitions: [
                    "public awareness",
                ],
                examples: [
                    "\"it brought the scandal to light\"",
                ],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05619850-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620592-n",
                part_of_speech: "n",
                definitions: [
                    "information that has become public",
                ],
                examples: [
                    "\"all the reports were out in the open\"",
                    "\"the facts had been brought to the surface\"",
                ],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05619850-n",
                    },
                ],
            },
            Synset {
                id: "ewn-05620826-n",
                part_of_speech: "n",
                definitions: [
                    "a young mind not yet affected by experience (according to John Locke)",
                ],
                examples: [],
                relations: [
                    SynsetRelation {
                        rel_type: "hypernym",
                        target: "ewn-05619057-n",
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
