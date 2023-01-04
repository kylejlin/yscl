macro_rules! atom {
    ($value:literal) => {{
        use $crate::*;
        Node::Atom(Atom {
            value: $value.to_string(),
        })
    }};
}

macro_rules! map {
    ($($key:ident = $value:expr),*) => {{
        use $crate::*;
        Node::Map(Map {
            entries: vec![$(MapEntry {
                key: Identifier::new(stringify!($key).to_string()).expect("Invalid identifier name"),
                value: $value,
            }),*],
        })
    }};
}

macro_rules! list {
    ($($element:expr),*) => {{
        use $crate::*;
        Node::List(List {
            elements: vec![$($element),*],
        })
    }};
}

macro_rules! parse {
    ($value:literal) => {
        atom!($value)
    };
    ({$($key:ident = $value:tt),*}) => {
        map! {
            $($key = parse!($value)),*
        }
    };
    ([$($element:tt),*]) => {
        list! {
            $(parse!($element)),*
        }
    };
}
