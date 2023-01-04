use crate::tree as finished;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Unfinished {
    AtomValue(String),
    List(UnfinishedList),
    Map(UnfinishedMap),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnfinishedList {
    pub elements: Vec<finished::Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnfinishedMap {
    pub entries: Vec<finished::MapEntry>,
    pub pending_entry: UnfinishedMapEntry,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnfinishedMapEntry {
    pub key: String,
    pub key_start_byte_index: Option<usize>,
    pub has_space_after_key: bool,
    pub has_equal: bool,
}

impl UnfinishedMapEntry {
    pub fn empty() -> Self {
        Self {
            key: "".to_string(),
            key_start_byte_index: None,
            has_space_after_key: false,
            has_equal: false,
        }
    }
}
