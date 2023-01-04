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
