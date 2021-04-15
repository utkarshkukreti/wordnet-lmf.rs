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
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
}

impl File {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, quick_xml::de::DeError> {
        use serde::de::Error;
        let file = std::fs::File::open(path).map_err(quick_xml::de::DeError::custom)?;
        quick_xml::de::from_reader(std::io::BufReader::new(file))
    }
}
