macro_rules! atom {
    ($value:literal) => {
        Node::Atom(Atom {
            value: $value.to_string(),
        })
    };
}

macro_rules! map {
    ($($key:literal = $value:expr),*) => {
        Node::Map(Map {
            entries: vec![$(MapEntry {
                key: Identifier::new($key.to_string()).expect("Invalid identifier name"),
                value: $value,
            }),*],
        })
    };
}

macro_rules! list {
    ($($element:expr),*) => {
        Node::List(List {
            elements: vec![$($element),*],
        })
    };
}
