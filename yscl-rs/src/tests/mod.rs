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
