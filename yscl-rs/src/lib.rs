//! A fast and simple parser for [YSCL](https://github.com/kylejlin/yscl).
//!
//! ## Example
//! ```rust
//! use yscl::prelude::*;
//!
//! let map = parse_doc(r#"
//! kantu_version = "1.0.0"
//! name = "fibonacci"
//! license = [
//!     "MIT"
//!     "Apache-2.0"
//! ]
//! dependencies = {
//!     yscl = "1.0.0"
//!     json = "1.0.0"
//! }
//! author = "xeklan (黒🐑)"
//! "#).unwrap();
//!
//! let expected = Map {
//!     entries: vec![
//!         MapEntry {
//!             key: Identifier::new("kantu_version".to_owned()).unwrap(),
//!             value: Node::Atom(
//!                 Atom {
//!                     value: "1.0.0".to_owned(),
//!                 },
//!             ),
//!         },
//!         MapEntry {
//!             key: Identifier::new("name".to_owned()).unwrap(),
//!             value: Node::Atom(
//!                 Atom {
//!                     value: "fibonacci".to_owned(),
//!                 },
//!             ),
//!         },
//!         MapEntry {
//!             key: Identifier::new("license".to_owned()).unwrap(),
//!             value: Node::List(
//!                 List {
//!                     elements: vec![
//!                         Node::Atom(
//!                             Atom {
//!                                 value: "MIT".to_owned(),
//!                             },
//!                         ),
//!                         Node::Atom(
//!                             Atom {
//!                                 value: "Apache-2.0".to_owned(),
//!                             },
//!                         ),
//!                     ],
//!                 },
//!             ),
//!         },
//!         MapEntry {
//!             key: Identifier::new("dependencies".to_owned()).unwrap(),
//!             value: Node::Map(
//!                 Map {
//!                     entries: vec![
//!                         MapEntry {
//!                             key: Identifier::new("yscl".to_owned()).unwrap(),
//!                             value: Node::Atom(
//!                                 Atom {
//!                                     value: "1.0.0".to_owned(),
//!                                 },
//!                             ),
//!                         },
//!                         MapEntry {
//!                             key: Identifier::new("json".to_owned()).unwrap(),
//!                             value: Node::Atom(
//!                                 Atom {
//!                                     value: "1.0.0".to_owned(),
//!                                 },
//!                             ),
//!                         },
//!                     ],
//!                 },
//!             ),
//!         },
//!         MapEntry {
//!             key: Identifier::new("author".to_owned()).unwrap(),
//!             value: Node::Atom(
//!                 Atom {
//!                     value: "xeklan (黒🐑)".to_owned(),
//!                 },
//!             ),
//!         },
//!     ],
//! };
//!
//! assert_eq!(expected, map);
//! ```

#[macro_use]
pub mod macros;

pub mod parse;
pub mod tree;

pub mod prelude {
    pub use crate::{macros::*, parse::*, tree::*};

    pub use crate::yscl_node;
}

#[cfg(test)]
mod tests;
