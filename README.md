# Yet Another Simple Config Language (YSCL)

YSCL (rhymes with "rascal") is a general purpose config language that
aims to be so simple that anyone can go
from zero to absolute mastery in less than five minutes.

YSCL files end with the `.yscl` extension.

## Learn by example

`hello_world.yscl`:

```yscl
// Hi, I'm a comment! I start with two slashes.
// I am ignored by the parser.
// I must go at the beginning of a line
   // (leading whitespace is permitted, however).




// Below is an entry. Every file has zero or more of them.
kantu_version = "1.0.0"

// Every entry has a _key_ and a _value_
//
// The key must be an _identifier_.
// An identifier contains one or more
// (ASCII) letters (uppercase or lowercase),
// digits, or underscores.
// It cannot begin with a digit.
//
// The value can be any YSCL expression.
// That is, it can be any of the following:
//
// 1. A string (for example, the "1.0.0" you saw above).
//    Strings are also called atoms.
// 2. A map (explained later)
// 3. A list (explained later)




// Below is a map.
// A map is a sequence of entries enclosed in curly braces (`{}`)
dependencies = {
    foo = "2.0.3"
    bar = "bar"

    lorem = {
        url = "https://github.com/kylejlin/nonexistent_repo"
    }

    // Note: There can only be one entry per line.
}




// Below is a list.
// A list is a sequence of elements enclosed in square brackets (`[]`).
licenses = [
    "MIT"
    // Elements are separated by newlines.
    "APACHE"
    // Elements can be any YSCL expression.
    {
        url = "https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE"
    }

    // Note: There can only be one element per line.
]




// There are 4 supported string escape sequences:
sequences = [
  // Double quote
  "\""

  // Backslash
  "\\"

  // Newline
  "\n"

  // Arbitrary Unicode Scalar Value
  "\u00263A"
  // You can replace `u00263A` with any 6 hexadecimal characters.
]
```

## Antipatterns and how to fix them

The following antipatterns constitute illegal YSCL code,
and will thus result in a parsing error.

### WRONG: Multiple entries per line

```yscl
foo = "bar" lorem = "ipsum"
```

### RIGHT: One entry per line

```yscl
foo = "bar"
lorem = "ipsum"
```

### WRONG: Multiple elements per line

```yscl
foo = ["bar" "baz"]
```

### RIGHT: One element per line

```yscl
foo = [
    "bar"
    "baz"
]
```

### WRONG: "One-liner" maps (and lists)

```yscl
lorem = { ipsum = "dolor" }
foo = ["bar"]
```

The enclosing `{}` (or `[]`, respectively) must be on different lines
than any entries (or elements, respectively) they enclose.

In the above example:

- The entry `ipsum = "dolor"` is illegal because it is on the same line as the enclosing `{`
- Likewise, the enclosing `}` is also illegal because it is on the same line as the entry `ipsum = "dolor"`
- The element `"bar"` is illegal because it is on the same line as the enclosing `[`
- Likewise, the enclosing `]` is also illegal because it is on the same line
  as the element `bar`.

### RIGHT: Entries and elements on their own line

```yscl
lorem = {
    ipsum = "dolor"
}
foo = [
    "bar"
]
```

### WRONG: Unicode surrogate code points in `\u` escapes

```yscl
foo = "\u00D83D\u00DE0A"
```

### RIGHT: Unicode scalar values in `\u` escapes

```yscl
foo = "\u01f60a"

// For this specific case, you could just directly write
// the value without escaping--that is,
foo2 = "😊"
```

### WRONG: Newline between an entry's key and the start of its value

```yscl
foo
    = "bar"

lorem =
{
    ipsum = "dolor"
}
```

### RIGHT: An entry's value starts on the same line as its key

```yscl
foo = "bar"

lorem = {
    ipsum = "dolor"
}
```

### WRONG: Duplicate keys

```yscl
license = "MIT"
license = "APACHE"
```

### RIGHT: ?

There isn't a one-size-fits-all solution for this.
The best solution will depend on what your goal is.

In the above example, it looks like the author is trying
to provide multiple licenses.
In this case, using a list would be appropriate:

```yscl
license = [
    "MIT"
    "APACHE"
]
```

Note that the same key may legally appear multiple times in
the same _file_, just not in the same _map_.
For example, the following is perfectly legal:

```yscl
jane_doe = {
    age = "21"
}

taro_yamada = {
    age = "24"
}
```

### WRONG: Comments on the same line as code

```yscl
foo = "bar" // Illegal comment
```

### RIGHT: Comments on their own line

```yscl
foo = "bar"
// This comment is legal
     // So is this one, although its indentation is strange.
```

## Parser implementations

TODO: Add link to `yscl-rs` on crates.io (once it's published).
