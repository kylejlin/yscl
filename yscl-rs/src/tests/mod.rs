use super::parse::parse;

fn assert_entry_count_eq(src: &str, expected_entry_count: usize) {
    match parse(src) {
        Ok(map) => {
            assert_eq!(expected_entry_count, map.entries.len());
        }
        Err(err_index) => {
            let remaining = src
                .char_indices()
                .filter_map(|(i, c)| if i >= err_index { Some(c) } else { None })
                .collect::<String>();
            panic!(
                "Err at index: {}\n\nREMAINING_SOURCE: {}\n\nCOMPLETE_SOURCE: {}",
                err_index, remaining, src,
            );
        }
    }
}

#[test]
fn hello_world() {
    let src = include_str!("sample_code/correct/hello_world.yscl");
    assert_entry_count_eq(src, 4);
}
