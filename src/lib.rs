pub type S = Box<str>;

pub type V<T> = Box<[T]>;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct File {
    #[serde(rename = "Lexicon")]
    pub lexicons: V<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lexicon {
    pub id: S,
    pub label: S,
    pub language: S,
    pub email: S,
    pub license: S,
    pub version: S,
    pub url: S,

    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: V<LexicalEntry>,

    #[serde(rename = "Synset")]
    pub synsets: V<Synset>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: S,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Form", default)]
    pub forms: V<Form>,
    #[serde(rename = "Sense")]
    pub senses: V<Sense>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Form {
    #[serde(rename = "writtenForm")]
    pub written_form: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Sense {
    pub id: S,
    pub synset: S,
    #[serde(rename = "SenseRelation", default)]
    pub relations: V<SenseRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SenseRelation {
    #[serde(rename = "relType")]
    pub rel_type: S,
    pub target: S,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Synset {
    pub id: S,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: S,
    #[serde(rename = "Definition")]
    pub definitions: V<S>,
    #[serde(rename = "Example", default)]
    pub examples: V<S>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: V<SynsetRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SynsetRelation {
    #[serde(rename = "relType")]
    pub rel_type: S,
    pub target: S,
}

impl File {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, quick_xml::de::DeError> {
        use serde::de::Error;
        let file = std::fs::File::open(path).map_err(quick_xml::de::DeError::custom)?;
        quick_xml::de::from_reader(std::io::BufReader::new(file))
    }
}
