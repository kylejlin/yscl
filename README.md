# Yet Another Simple Config Language (YSCL)

YSCL (rhymes with "Haskell" and "rascal") is a simple config language that anyone can learn in five minutes.

It's focus is on extreme simplicity and readability.

Example:

`package.yscl`:

```yscl
name = "hangman"
version = "1.0.0"
dependencies = {
    random = "2.0.4"
    left_pad = "1.2.0"
}
licenses = [
    "MIT"
    "APACHE-2.0"
    {
        name = "Custom license"
        url = "https://github.com/kyljelin/nonexistent_repo/dummy_url.txt"
    }
]
```

## Why YSCL?

It's simple, readable, and quick to learn.

Of course, that can also be said of its competitors, so here are some comparisons:

vs. JSON

- JSON requires the entire file be enclosed in `{}`
  - This effectively forces the indentation to start off at 1
    indent (in contrast to TOML or YML which start at 0 indents).
  - YSCL doesn't require this.
- JSON requires keys to be enclosed with `""`, which is ugly.
  - YSCL doesn't require this.
- JSON has 4 atomic types (string, number, boolean, null).
  - YSCL has 1 (string).
  - Less types mean less checks. For example, in JavaScript, this allows you to simply write `x.endsWith(".xml")` instead of the more cumbersome `typeof x === "string" && x.endsWith("xml")`.

vs. YML

- YSCL is much simpler than YML
  - This admittedly is a weakness in certain domains, such as in CI
    scripting (YML's block strings are great for writing inline code)
  - However, it drastically reduces the amount of cases you need to check.

## Learn

Please read [CRASH_COURSE.md](./learn/CRASH_COURSE.md).

## Parser implementations

- Rust: [yscl-rs](https://crates.io/crates/yscl)
- Other languages: coming soon...hopefully. It depends on demand.
  - If you're interested, please open an [issue](https://github.com/kylejlin/yscl/issues/new), and I can write a port in your desired language, assuming it's a reasonably common language (sorry, I'm not writing a [LOLCODE](https://en.wikipedia.org/wiki/LOLCODE) port--but feel free to do so yourself)
- Even better, you can implement a parser yourself!
  - If you open an [issue](https://github.com/kylejlin/yscl/issues/new),
    I will add your parser to this list of implementations.
  - I will also add you to the contributors list (which currently doesn't exist since it's just me 😆).
