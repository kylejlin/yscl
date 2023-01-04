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
    expected_success(src, &expected);
}
