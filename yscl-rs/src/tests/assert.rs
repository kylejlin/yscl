use super::*;

const INDENT_INCREMENT: usize = 4;

pub fn expect_success(src: &str, expected: &Node) {
    match parse(src) {
        Ok(map) => {
            assert_node_eq(expected, &Node::Map(map));
        }
        Err(ParseError::UnexpectedChar(unexpected_c, unexpected_index)) => {
            let remaining = src
                .char_indices()
                .filter_map(|(i, c)| if i >= unexpected_index { Some(c) } else { None })
                .collect::<String>();
            panic!(
                "Error at index {}: Unexpected: {}\n\nREMAINING_SOURCE: {}\n\nCOMPLETE_SOURCE: {}",
                unexpected_index, unexpected_c, remaining, src,
            );
        }
        Err(err) => println!("Error: {:?}", err),
    }
}

fn assert_node_eq(expected: &Node, actual: &Node) {
    if expected == actual {
        return;
    }

    panic!(
        "NODE MISMATCH\nCOMMON:\n\n{}\n\nEXPECTED:\n\n{}\n\nACTUAL:\n\n{}",
        get_commonality(expected, actual),
        format(expected),
        format(actual)
    );
}

fn get_commonality(left: &Node, right: &Node) -> String {
    let mut out = "".to_string();
    std::mem::drop(write_commonality(&mut out, left, right, 0));
    out
}

fn write_commonality(
    out: &mut String,
    left: &Node,
    right: &Node,
    indent_level: usize,
) -> Result<(), ()> {
    let i0 = " ".repeat(indent_level);
    let next_indent_level = indent_level + INDENT_INCREMENT;
    let i1 = " ".repeat(next_indent_level);

    match (left, right) {
        (Node::Atom(left), Node::Atom(right)) => {
            if left.value == right.value {
                out.push_str(&format!("{:?}", left.value));
            } else {
                return Err(());
            }
        }
        (Node::Map(left), Node::Map(right)) => {
            out.push_str("{");
            let mut left_entries = left.entries.iter();
            let mut right_entries = right.entries.iter();
            loop {
                match (left_entries.next(), right_entries.next()) {
                    (Some(left_entry), Some(right_entry)) => {
                        if left_entry.key != right_entry.key {
                            return Err(());
                        }
                        out.push_str(&format!("\n{}{} = ", i1, left_entry.key.as_str()));
                        write_commonality(
                            out,
                            &left_entry.value,
                            &right_entry.value,
                            next_indent_level,
                        )?;
                    }
                    (None, None) => break,
                    _ => return Err(()),
                }
            }
            out.push_str(&format!("\n{}}}\n", i0));
        }
        (Node::List(left), Node::List(right)) => {
            out.push_str("[");
            let mut left_elements = left.elements.iter();
            let mut right_elements = right.elements.iter();
            loop {
                match (left_elements.next(), right_elements.next()) {
                    (Some(left_elem), Some(right_elem)) => {
                        out.push_str(&format!("\n{}", i1));
                        write_commonality(out, left_elem, right_elem, next_indent_level)?;
                    }
                    (None, None) => break,
                    _ => return Err(()),
                }
            }
            out.push_str(&format!("\n{}]\n", i0));
        }
        _ => {
            return Err(());
        }
    }
    Ok(())
}

fn format(node: &Node) -> String {
    get_commonality(node, node)
}

pub fn expect_unexpected_char_err(src: &str, expected_c: char) {
    match parse(src) {
        Ok(map) => {
            panic!(
                "Expected error, but unexpectedly parsed successfully. Map:\n\n{}",
                format(&Node::Map(map))
            )
        }
        Err(ParseError::UnexpectedChar(actual_c, actual_index)) => {
            assert_eq!(expected_c, actual_c);
            let actual =
                src.char_indices()
                    .find_map(|(i, c)| if i == actual_index { Some(c) } else { None });
            assert_eq!(
                Some(expected_c),
                actual,
                "Index {} does not match.\n\nsrc = {:?}",
                actual_index,
                src,
            );
        }
        Err(err) => println!("Got a different error than expected: {:?}", err),
    }
}

pub fn expect_duplicate_key_char_err(src: &str, expected_key: &str) {
    match parse(src) {
        Ok(map) => {
            panic!(
                "Expected error, but unexpectedly parsed successfully. Map:\n\n{}",
                format(&Node::Map(map))
            )
        }
        Err(ParseError::DuplicateKey(actual_key, actual_index)) => {
            assert_eq!(expected_key, actual_key);
            let actual: String = src
                .char_indices()
                .filter_map(|(i, c)| if i >= actual_index { Some(c) } else { None })
                .take(expected_key.chars().count())
                .collect();
            assert_eq!(
                expected_key,
                actual.as_str(),
                "Index {} does not match.\n\nsrc = {:?}",
                actual_index,
                src,
            );
        }
        Err(err) => println!("Got a different error than expected: {:?}", err),
    }
}
