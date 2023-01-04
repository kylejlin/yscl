macro_rules! yscl_node {
    ($value:literal) => {{
        use $crate::*;
        Node::Atom(Atom {
            value: $value.to_string(),
        })
    }};
    ({$($key:ident = $value:tt),*}) => {{
        use $crate::*;
        Node::Map(Map {
            entries: vec![$(MapEntry {
                key: Identifier::new(stringify!($key).to_string()).expect("Invalid identifier name"),
                value: yscl_node!($value),
            }),*],
        })
    }};
    ([$($element:tt),*]) => {{
        use $crate::*;
        Node::List(List {
            elements: vec![$(yscl_node!($element)),*],
        })
    }};
}
