/// A macro to create a YSCL node.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate yscl;
/// use yscl::*;
///
/// let atom = yscl_node!("hello");
/// assert_eq!(atom, Node::Atom(Atom { value: "hello".to_string() }));
///
/// let map = yscl_node!({hello = "world", this_is = "a map"});
/// assert_eq!(map, Node::Map(Map { entries: vec![
///     MapEntry {
///         key: Identifier::new("hello".to_string()).unwrap(),
///         value: Node::Atom(Atom { value: "world".to_string() }),
///     },
///     MapEntry {
///         key: Identifier::new("this_is".to_string()).unwrap(),
///         value: Node::Atom(Atom { value: "a map".to_string() }),
///     },
/// ]}));
///
/// let list = yscl_node!(["hello", "world"]);
/// assert_eq!(list, Node::List(List { elements: vec![
///     Node::Atom(Atom { value: "hello".to_string() }),
///    Node::Atom(Atom { value: "world".to_string() }),
/// ]}));
///
/// let complex = yscl_node!({
///     kantu_version = "1.0.0",
///     name = "fibonacci",
///     license = [
///         "MIT",
///         "Apache-2.0"
///     ],
///     dependencies = {
///         yscl = "1.0.0",
///         json = "1.0.0"
///     }
/// });
/// let complex_expected = parse_doc(r#"
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
/// "#).unwrap();
/// assert_eq!(complex, Node::Map(complex_expected));
/// ```
#[macro_export]
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
