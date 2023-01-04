use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Atom(AtomValue),
    List(List),
    Map(Map),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AtomValue(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Map {
    pub entries: Vec<MapEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MapEntry {
    pub key: Identifier,
    pub value: Node,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.0
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Identifier {
    /// If the input contains an illegal character, returns the index of the first illegal character.
    ///
    /// Legal characters are ASCII letters (both uppercase and lowercase), digits, and the underscore.
    pub fn new(s: String) -> Result<Self, usize> {
        let bad_char_pos = s.char_indices().find_map(|(i, c)| {
            if c.is_ascii_alphanumeric() || c == '_' {
                None
            } else {
                Some(i)
            }
        });
        if let Some(i) = bad_char_pos {
            Err(i)
        } else {
            Ok(Self(s))
        }
    }
}
