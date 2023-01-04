/// A macro for concisely constructing YSCL nodes.
///
/// ## Usage
///
/// ```rust
/// use yscl::*;
///
/// let atom = yscl_node!("hello");
/// let atom_verbose = Node::Atom(Atom { value: "hello".to_string() });
/// assert_eq!(atom, atom_verbose);
/// ```
///
/// ## More examples
///
/// Maps
///
/// ```rust
/// # use yscl::*;
/// let map = yscl_node!({
///     hello = "world",
///     this_is = "a map"
/// });
/// let map_verbose = Node::Map(Map { entries: vec![
///     MapEntry {
///         key: Identifier::new("hello".to_string()).unwrap(),
///         value: Node::Atom(Atom { value: "world".to_string() }),
///     },
///     MapEntry {
///         key: Identifier::new("this_is".to_string()).unwrap(),
///         value: Node::Atom(Atom { value: "a map".to_string() }),
///     },
/// ]});
/// assert_eq!(map, map_verbose);
/// ```
///
/// Lists
///
/// ```rust
/// # use yscl::*;
/// let list = yscl_node!([
///     "hello",
///     "world"
/// ]);
/// let list_verbose = Node::List(List { elements: vec![
///     Node::Atom(Atom { value: "hello".to_string() }),
///    Node::Atom(Atom { value: "world".to_string() }),
/// ]});
/// assert_eq!(list, list_verbose);
/// ```
///
/// Nested
///
/// ```rust
/// # use yscl::*;
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
///     },
///     author = "xeklan (黒🐑)"
/// });
///
/// let complex_verbose = Node::Map(Map {
///     entries: vec![
///         MapEntry {
///             key: Identifier::new("kantu_version".to_owned()).unwrap(),
///             value: Node::Atom(
///                 Atom {
///                     value: "1.0.0".to_owned(),
///                 },
///             ),
///         },
///         MapEntry {
///             key: Identifier::new("name".to_owned()).unwrap(),
///             value: Node::Atom(
///                 Atom {
///                     value: "fibonacci".to_owned(),
///                 },
///             ),
///         },
///         MapEntry {
///             key: Identifier::new("license".to_owned()).unwrap(),
///             value: Node::List(
///                 List {
///                     elements: vec![
///                         Node::Atom(
///                             Atom {
///                                 value: "MIT".to_owned(),
///                             },
///                         ),
///                         Node::Atom(
///                             Atom {
///                                 value: "Apache-2.0".to_owned(),
///                             },
///                         ),
///                     ],
///                 },
///             ),
///         },
///         MapEntry {
///             key: Identifier::new("dependencies".to_owned()).unwrap(),
///             value: Node::Map(
///                 Map {
///                     entries: vec![
///                         MapEntry {
///                             key: Identifier::new("yscl".to_owned()).unwrap(),
///                             value: Node::Atom(
///                                 Atom {
///                                     value: "1.0.0".to_owned(),
///                                 },
///                             ),
///                         },
///                         MapEntry {
///                             key: Identifier::new("json".to_owned()).unwrap(),
///                             value: Node::Atom(
///                                 Atom {
///                                     value: "1.0.0".to_owned(),
///                                 },
///                             ),
///                         },
///                     ],
///                 },
///             ),
///         },
///         MapEntry {
///             key: Identifier::new("author".to_owned()).unwrap(),
///             value: Node::Atom(
///                 Atom {
///                     value: "xeklan (黒🐑)".to_owned(),
///                 },
///             ),
///         },
///     ],
/// });
///
/// assert_eq!(complex, complex_verbose);
/// ```
///
/// ## Limitations
/// Trailing commas are currently not supported.
/// However, if someone wants to implement this, I'd be happy to accept a PR.
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
