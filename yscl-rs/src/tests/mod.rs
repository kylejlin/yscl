use crate::*;

mod assert;
use assert::*;

#[test]
fn hello_world() {
    let src = include_str!("sample_code/correct/hello_world.yscl");
    let expected = yscl_node! ({
        kantu_version = "1.0.0",
        dependencies = {
            foo = "2.0.3",
            bar = "bar",
            lorem = {
                url = "https://github.com/kylejlin/nonexistent_repo"
            }
        },
        licenses = [
            "MIT",
            "APACHE",
            {
                url = "https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE"
            }
        ],
        sequences = [
            "\"",
            "\\",
            "\n",
            "\u{263A}"
        ]
    });
    expect_success(src, &expected);
}

mod code_comment_same_line {
    use super::*;

    #[test]
    fn wrong() {
        let src =
            include_str!("sample_code/patterns_and_antipatterns/code_comment_same_line/wrong.yscl");
        expect_unexpected_char_err(src, '/');
    }

    #[test]
    fn right() {
        let src =
            include_str!("sample_code/patterns_and_antipatterns/code_comment_same_line/right.yscl");
        let expected = yscl_node!({ foo = "bar" });
        expect_success(src, &expected);
    }
}

mod duplicate_keys {
    use super::*;

    #[ignore]
    #[test]
    fn wrong_nested() {
        let src =
            include_str!("sample_code/patterns_and_antipatterns/duplicate_keys/wrong_nested.yscl");
        expect_duplicate_key_char_err(src, "license");
    }

    #[ignore]
    #[test]
    fn wrong_top_level() {
        let src = include_str!(
            "sample_code/patterns_and_antipatterns/duplicate_keys/wrong_top_level.yscl"
        );
        expect_duplicate_key_char_err(src, "license");
    }

    #[test]
    fn right_different_maps() {
        let src = include_str!(
            "sample_code/patterns_and_antipatterns/duplicate_keys/right_different_maps.yscl"
        );
        let expected = yscl_node!({ jane_doe = { age = "21" }, taro_yamada = { age = "24" } });
        expect_success(src, &expected);
    }

    #[test]
    fn right_list() {
        let src =
            include_str!("sample_code/patterns_and_antipatterns/duplicate_keys/right_list.yscl");
        let expected = yscl_node!({ license = ["MIT", "APACHE"] });
        expect_success(src, &expected);
    }
}
