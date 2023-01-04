use crate::*;

fn assert_entry_count_eq(src: &str, expected_entry_count: usize) {
    match parse(src) {
        Ok(map) => {
            assert_eq!(expected_entry_count, map.entries.len());
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

#[test]
fn hello_world() {
    let src = include_str!("sample_code/correct/hello_world.yscl");
    assert_entry_count_eq(src, 4);
}
