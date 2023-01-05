use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Atom(Atom),
    Map(Map),
    List(List),
}

impl Node {
    pub fn atom(self) -> Option<Atom> {
        match self {
            Node::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    pub fn map(self) -> Option<Map> {
        match self {
            Node::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn list(self) -> Option<List> {
        match self {
            Node::List(list) => Some(list),
            _ => None,
        }
    }
}

impl Node {
    pub fn as_ref(&self) -> NodeRef {
        match self {
            Node::Atom(atom) => NodeRef::Atom(atom),
            Node::Map(map) => NodeRef::Map(map),
            Node::List(list) => NodeRef::List(list),
        }
    }
}

impl From<Atom> for Node {
    fn from(atom: Atom) -> Self {
        Node::Atom(atom)
    }
}

impl From<Map> for Node {
    fn from(map: Map) -> Self {
        Node::Map(map)
    }
}

impl From<List> for Node {
    fn from(list: List) -> Self {
        Node::List(list)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeRef<'a> {
    Atom(&'a Atom),
    Map(&'a Map),
    List(&'a List),
}

impl<'a> NodeRef<'a> {
    pub fn atom(self) -> Option<&'a Atom> {
        match self {
            NodeRef::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    pub fn map(self) -> Option<&'a Map> {
        match self {
            NodeRef::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn list(self) -> Option<&'a List> {
        match self {
            NodeRef::List(list) => Some(list),
            _ => None,
        }
    }
}

impl NodeRef<'_> {
    pub fn to_owned(self) -> Node {
        match self {
            NodeRef::Atom(atom) => Node::Atom(atom.clone()),
            NodeRef::Map(map) => Node::Map(map.clone()),
            NodeRef::List(list) => Node::List(list.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Atom {
    /// This is the _value_ of the atom, not the _source_.
    /// For example, the YSCL atom `"\""` will be represented
    /// by the Rust expression `Atom { value: "\"".to_string() }`,
    /// NOT `Atom { value: "\"\\\"\"".to_string() }`.
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Map {
    pub entries: Vec<MapEntry>,
}

impl Map {
    /// Get the value corresponding to the given key
    /// if one exists.
    ///
    /// ## Example
    /// ```rust
    /// # use yscl::prelude::*;
    /// let map = parse_doc(r#"
    /// kantu_version = "1.0.0"
    /// name = "fibonacci"
    /// license = [
    ///     "MIT"
    ///     "Apache-2.0"
    /// ]
    /// dependencies = {
    ///     yscl = "1.0.0"
    ///     json = "1.0.0"
    /// }
    /// author = "xeklan (黒🐑)"
    /// "#).unwrap();
    /// assert_eq!(
    ///     map.get("kantu_version").unwrap().as_ref().atom().unwrap().value,
    ///     "1.0.0",
    /// );
    /// assert_eq!(
    ///     map.get("repository"),
    ///     None,
    /// );
    /// ```
    pub fn get<K>(&self, key: &K) -> Option<&Node>
    where
        K: ?Sized + AsRef<str>,
    {
        self.entries.iter().find_map(|entry| {
            if *entry.key == *key.as_ref() {
                Some(&entry.value)
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MapEntry {
    pub key: Identifier,
    pub value: Node,
}

/// A string consisting of one or more ASCII letters,
/// digits, or underscores.
/// The initial character may **not** be a digit.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl From<Identifier> for String {
    fn from(identifier: Identifier) -> Self {
        identifier.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Identifier {
    /// If the input contains an illegal character, returns the index of the first illegal character.
    ///
    /// Legal characters are ASCII letters (both uppercase and lowercase), digits, and the underscore.
    pub fn new(s: String) -> Result<Self, usize> {
        if let Some(first) = s.chars().next() {
            if first.is_ascii_digit() {
                return Err(0);
            }
        }

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
