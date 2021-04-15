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
}

impl File {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, quick_xml::de::DeError> {
        use serde::de::Error;
        let file = std::fs::File::open(path).map_err(quick_xml::de::DeError::custom)?;
        quick_xml::de::from_reader(std::io::BufReader::new(file))
    }
}
