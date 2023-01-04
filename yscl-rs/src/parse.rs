use crate::tree::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The character and its byte position.
    UnexpectedChar(char, usize),
    UnexpectedEoi,
    DuplicateKey(String, usize),
}

pub fn parse(src: &str) -> Result<Map, ParseError> {
    let unexpected_eoi_err = Err(ParseError::UnexpectedEoi);
    let mut stack = vec![Unfinished::Map(UnfinishedMap {
        entries: vec![],
        pending_entry: UnfinishedMapEntry::empty(),
    })];
    let mut remaining = wrap_in_non_whitespace_tracker(src.char_indices());

    while let Some((i, c)) = remaining.next() {
        match stack.last_mut().expect("Stack should never be empty") {
            Unfinished::AtomValue(atom_value) => match c {
                '\n' => return Err(ParseError::UnexpectedChar(c, i)),
                '"' => {
                    let top = Node::Atom(Atom {
                        value: atom_value.clone(),
                    });
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '\\' => {
                    let Some((i_of_c_after_backslash, c_after_backslash)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match c_after_backslash {
                        '\\' | '"' => atom_value.push(c_after_backslash),
                        'n' => atom_value.push('\n'),
                        'u' => {
                            let mut hex = String::with_capacity(6);
                            for _ in 0..5 {
                                let Some((hex_i, hex_c)) = remaining.next() else {
                                    return unexpected_eoi_err;
                                };
                                if !hex_c.is_ascii_hexdigit() {
                                    return Err(ParseError::UnexpectedChar(hex_c, hex_i));
                                }
                                hex.push(hex_c);
                            }
                            let (last_hex_i, last_hex_c) = if let Some((last_hex_i, last_hex_c)) =
                                remaining.next()
                            {
                                if !last_hex_c.is_ascii_hexdigit() {
                                    return Err(ParseError::UnexpectedChar(last_hex_c, last_hex_i));
                                }
                                hex.push(last_hex_c);
                                (last_hex_i, last_hex_c)
                            } else {
                                return unexpected_eoi_err;
                            };
                            let codepoint = u32::from_str_radix(&hex, 16)
                                .expect("Hex code should always be valid");
                            let Some(encoded_char) = std::char::from_u32(codepoint) else {
                                return Err(ParseError::UnexpectedChar(last_hex_c, last_hex_i));
                            };
                            atom_value.push(encoded_char);
                        }
                        _ => {
                            return Err(ParseError::UnexpectedChar(
                                c_after_backslash,
                                i_of_c_after_backslash,
                            ))
                        }
                    }
                }
                _other_char => atom_value.push(c),
            },

            Unfinished::List(UnfinishedList { elements }) => match c {
                ']' if remaining.non_whitespace_on_current_line() == 1 || elements.is_empty() => {
                    let top = Node::List(List {
                        elements: elements.clone(),
                    });
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '"' if remaining.non_whitespace_on_current_line() == 1 => {
                    stack.push(Unfinished::AtomValue("".to_string()));
                }
                '{' if remaining.non_whitespace_on_current_line() == 1 => {
                    stack.push(Unfinished::Map(UnfinishedMap {
                        entries: vec![],
                        pending_entry: UnfinishedMapEntry::empty(),
                    }));
                }
                '[' if remaining.non_whitespace_on_current_line() == 1 => {
                    stack.push(Unfinished::List(UnfinishedList { elements: vec![] }));
                }
                '/' if remaining.non_whitespace_on_current_line() == 1 => {
                    let Some((next_i, next_c)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match next_c {
                        '/' => {
                            while let Some((_, next_c)) = remaining.next() {
                                if next_c == '\n' {
                                    break;
                                }
                            }
                        }
                        _ => return Err(ParseError::UnexpectedChar(next_c, next_i)),
                    }
                }
                c if c.is_whitespace() => {}
                _ => return Err(ParseError::UnexpectedChar(c, i)),
            },

            Unfinished::Map(UnfinishedMap {
                entries,
                pending_entry,
            }) => match c {
                '}' if remaining.non_whitespace_on_current_line() == 1 || entries.is_empty() => {
                    if !pending_entry.key.is_empty() {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }
                    let top = Node::Map(Map {
                        entries: entries.clone(),
                    });
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '\n' => {
                    if !pending_entry.key.is_empty() {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }
                }
                '=' => {
                    let Some(start) = pending_entry.key_start_byte_index else {
                        return Err(ParseError::UnexpectedChar(c, i));
                    };

                    if entries
                        .iter()
                        .any(|existing_entry| *existing_entry.key == pending_entry.key)
                    {
                        return Err(ParseError::DuplicateKey(pending_entry.key.clone(), start));
                    }

                    if pending_entry.has_equal {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }

                    pending_entry.has_equal = true;
                }
                c if c.is_whitespace() => {
                    if !pending_entry.key.is_empty() {
                        pending_entry.has_space_after_key = true;
                    }
                }
                c if is_identifier_char(c) && pending_entry.key.is_empty() => {
                    // Entries must be on their own line.
                    if remaining.non_whitespace_on_current_line() != 1 {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }

                    // Leading digits are forbidden.
                    if c.is_ascii_digit() {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }

                    let can_push = !pending_entry.has_space_after_key && !pending_entry.has_equal;
                    if !can_push {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }

                    pending_entry.key_start_byte_index = Some(i);
                    pending_entry.key.push(c);
                }
                c if is_identifier_char(c) => {
                    let can_push = !pending_entry.has_space_after_key && !pending_entry.has_equal;
                    if !can_push {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }

                    pending_entry.key.push(c);
                }
                '"' => {
                    if !pending_entry.has_equal {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }
                    stack.push(Unfinished::AtomValue("".to_string()));
                }
                '{' => {
                    if !pending_entry.has_equal {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }
                    stack.push(Unfinished::Map(UnfinishedMap {
                        entries: vec![],
                        pending_entry: UnfinishedMapEntry::empty(),
                    }));
                }
                '[' => {
                    if !pending_entry.has_equal {
                        return Err(ParseError::UnexpectedChar(c, i));
                    }
                    stack.push(Unfinished::List(UnfinishedList { elements: vec![] }));
                }
                '/' if remaining.non_whitespace_on_current_line() == 1 => {
                    let Some((next_i, next_c)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match next_c {
                        '/' => {
                            while let Some((_, next_c)) = remaining.next() {
                                if next_c == '\n' {
                                    break;
                                }
                            }
                        }
                        _ => return Err(ParseError::UnexpectedChar(next_c, next_i)),
                    }
                }
                _ => return Err(ParseError::UnexpectedChar(c, i)),
            },
        }
    }

    let last = stack.pop();
    let new_len = stack.len();
    match (last, new_len) {
        (
            Some(Unfinished::Map(UnfinishedMap {
                entries,
                pending_entry:
                    UnfinishedMapEntry {
                        key,
                        key_start_byte_index: _,
                        has_equal: _,
                        has_space_after_key: _,
                    },
            })),
            0,
        ) if key.is_empty() => Ok(Map { entries }),
        _ => unexpected_eoi_err,
    }
}

#[derive(Debug, Clone)]
struct NonWhiteSpaceTracker<I> {
    iter: I,
    non_whitespace_on_current_line: usize,
}

impl<I> NonWhiteSpaceTracker<I> {
    pub fn non_whitespace_on_current_line(&self) -> usize {
        self.non_whitespace_on_current_line
    }
}

fn wrap_in_non_whitespace_tracker<I: Iterator<Item = (usize, char)>>(
    iter: I,
) -> NonWhiteSpaceTracker<I> {
    NonWhiteSpaceTracker {
        iter,
        non_whitespace_on_current_line: 0,
    }
}

impl<I> Iterator for NonWhiteSpaceTracker<I>
where
    I: Iterator<Item = (usize, char)>,
{
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let Some((i, c)) = self.iter.next() else {
            return None;
        };
        if c == '\n' {
            self.non_whitespace_on_current_line = 0;
        } else if !c.is_whitespace() {
            self.non_whitespace_on_current_line += 1;
        }

        Some((i, c))
    }
}

fn reduce(stack: &mut Vec<Unfinished>, top: Node) -> Result<Option<Map>, ()> {
    match stack.last_mut() {
        None => match top {
            Node::Map(top) => Ok(Some(top)),
            _ => Err(()),
        },
        Some(Unfinished::AtomValue(_)) => Err(()),
        Some(Unfinished::List(UnfinishedList { elements })) => {
            elements.push(top);
            Ok(None)
        }
        Some(Unfinished::Map(UnfinishedMap {
            entries,
            pending_entry,
        })) => {
            if pending_entry.has_equal {
                entries.push(MapEntry {
                    key: Identifier::new(pending_entry.key.clone())
                        .expect("Pending key should always be valid"),
                    value: top,
                });

                *pending_entry = UnfinishedMapEntry::empty();

                Ok(None)
            } else {
                Err(())
            }
        }
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

const REDUCE_SHOULD_SUCCEED_MSG: &str = "Reduce should never fail, since we only ever push a node to the stack when the item under it is ready for it.";

use unfinished::*;
mod unfinished {
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
}
