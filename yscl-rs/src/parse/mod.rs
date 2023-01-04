use crate::tree::*;

mod non_whitespace_tracker;
use non_whitespace_tracker::*;

mod reduce;
use reduce::*;

mod unfinished;
use unfinished::*;

const REDUCE_SHOULD_SUCCEED_MSG: &str = "Reduce should never fail, since we only ever push a node to the stack when the item under it is ready for it.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The character and its byte position.
    UnexpectedChar(char, usize),
    UnexpectedEoi,
    DuplicateKey(String, usize),
}

pub fn parse(src: &str) -> Result<Map, ParseError> {
    let mut stack = vec![Unfinished::Map(UnfinishedMap {
        entries: vec![],
        pending_entry: UnfinishedMapEntry::empty(),
    })];
    let mut remaining = wrap_in_non_whitespace_tracker(src.char_indices());

    while let Some((i, c)) = remaining.next() {
        if let Continuation::Return(return_val) =
            handle_character(&mut stack, i, c, &mut remaining)?
        {
            return Ok(return_val);
        }
    }

    handle_eoi(&mut stack)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Continuation<T> {
    Return(T),
    Continue,
}

fn handle_character<I>(
    stack: &mut Vec<Unfinished>,
    i: usize,
    c: char,
    remaining: &mut NonWhiteSpaceTracker<I>,
) -> Result<Continuation<Map>, ParseError>
where
    I: Iterator<Item = (usize, char)>,
{
    match stack.last_mut().expect("Stack should never be empty") {
        Unfinished::AtomValue(atom_value) => match c {
            '\n' => return Err(ParseError::UnexpectedChar(c, i)),
            '"' => {
                let top = Node::Atom(Atom {
                    value: atom_value.clone(),
                });
                stack.pop().unwrap();
                if let Some(return_val) = reduce_stack(stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                {
                    return Ok(Continuation::Return(return_val));
                }
            }
            '\\' => {
                let Some((i_of_c_after_backslash, c_after_backslash)) = remaining.next() else {
                    return Err(ParseError::UnexpectedEoi);
                };
                match c_after_backslash {
                    '\\' | '"' => atom_value.push(c_after_backslash),
                    'n' => atom_value.push('\n'),
                    'u' => {
                        let mut hex = String::with_capacity(6);
                        for _ in 0..5 {
                            let Some((hex_i, hex_c)) = remaining.next() else {
                                return Err(ParseError::UnexpectedEoi);
                            };
                            if !hex_c.is_ascii_hexdigit() {
                                return Err(ParseError::UnexpectedChar(hex_c, hex_i));
                            }
                            hex.push(hex_c);
                        }
                        let (last_hex_i, last_hex_c) =
                            if let Some((last_hex_i, last_hex_c)) = remaining.next() {
                                if !last_hex_c.is_ascii_hexdigit() {
                                    return Err(ParseError::UnexpectedChar(last_hex_c, last_hex_i));
                                }
                                hex.push(last_hex_c);
                                (last_hex_i, last_hex_c)
                            } else {
                                return Err(ParseError::UnexpectedEoi);
                            };
                        let codepoint =
                            u32::from_str_radix(&hex, 16).expect("Hex code should always be valid");
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
                if let Some(return_val) = reduce_stack(stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                {
                    return Ok(Continuation::Return(return_val));
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
                    return Err(ParseError::UnexpectedEoi);
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

        Unfinished::List(UnfinishedList { elements }) => match c {
            ']' if remaining.non_whitespace_on_current_line() == 1 || elements.is_empty() => {
                let top = Node::List(List {
                    elements: elements.clone(),
                });
                stack.pop().unwrap();
                if let Some(return_val) = reduce_stack(stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                {
                    return Ok(Continuation::Return(return_val));
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
                    return Err(ParseError::UnexpectedEoi);
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
    }

    Ok(Continuation::Continue)
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn handle_eoi(stack: &mut Vec<Unfinished>) -> Result<Map, ParseError> {
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
        _ => Err(ParseError::UnexpectedEoi),
    }
}
