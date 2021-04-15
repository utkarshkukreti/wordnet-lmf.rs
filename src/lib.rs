#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct File {
    #[serde(rename = "Lexicon")]
    pub lexicons: Vec<Lexicon>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lexicon {
    pub id: String,
    pub label: String,
    pub language: String,
    pub email: String,
    pub license: String,
    pub version: String,
    pub url: String,

    #[serde(rename = "LexicalEntry")]
    pub lexical_entries: Vec<LexicalEntry>,

    #[serde(rename = "Synset")]
    pub synsets: Vec<Synset>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Form", default)]
    pub forms: Vec<Form>,
    #[serde(rename = "Sense")]
    pub senses: Vec<Sense>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Form {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Sense {
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub relations: Vec<SenseRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SenseRelation {
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Synset {
    pub id: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    #[serde(rename = "Definition")]
    pub definitions: Vec<String>,
    #[serde(rename = "Example", default)]
    pub examples: Vec<String>,
    #[serde(rename = "SynsetRelation", default)]
    pub relations: Vec<SynsetRelation>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct SynsetRelation {
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,
}

impl File {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, quick_xml::de::DeError> {
        use serde::de::Error;
        let file = std::fs::File::open(path).map_err(quick_xml::de::DeError::custom)?;
        quick_xml::de::from_reader(std::io::BufReader::new(file))
    }
}
