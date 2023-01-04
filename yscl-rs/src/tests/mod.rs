use crate::*;

#[macro_use]
mod macros;

mod assert;
use assert::*;

#[test]
fn hello_world() {
    let src = include_str!("sample_code/correct/hello_world.yscl");
    let expected = map! {
        "kantu_version" = atom!("1.0.0"),
        "dependencies" = map! {
            "foo" = atom!("2.0.3"),
            "bar" = atom!("bar"),
            "lorem" = map! {
                "url" = atom!("https://github.com/kylejlin/nonexistent_repo")
            }
        },
        "licenses" = list! [
            atom!("MIT"),
            atom!("APACHE"),
            map! {
                "url" = atom!("https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE")
            }
        ],
        "sequences" = list! [
            atom!("\""),
            atom!("\\"),
            atom!("\n"),
            atom!("\u{263A}")
        ]
    };
    expected_success(src, &expected);
}
