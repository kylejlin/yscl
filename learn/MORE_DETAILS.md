# YSCL - More details

## Learn by counterexample

Below, we will present a series of illegal code samples (WRONG)
and examples of the correct equivalent (RIGHT).

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

Map entries must be on their own line--they cannot be on
the same line as either of their enclosing curly braces (`{}`).

The same restriction applies to list elements and their
enclosing square brackets (`[]`).

There are no exceptions.

### RIGHT: Entries and elements on their own line

```yscl
lorem = {
    ipsum = "dolor"
}
foo = [
    "bar"
]

empty_map = {}
empty_list = []
```

Observe that since _empty_ maps don't have any entries,
they can be written on one line.

Note that this is not an exception to the "one line per entry" rule,
but rather a particular case of it.
Since all zero entries are on their own line, the rule is
vacuously satisfied.

The same applies to empty lists.

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

## Glossary

- **File/document**: The entirety of the code.
  - A file consists of a sequence of entries.
  - The entries cannot have duplicate keys
- **Expression**: An atom (string), map, or list.
- **Atom**: a string
- **Identifier**: one or more letters, digits, or underscores.
  The first character cannot be a digit.
- **Entry**: a key (identifier) and a value (expression).
  - There is an `=` in between the key and value.
  - A map consists of zero or more entries.
  - A file also consists of zero or more entries.
    A file is semantically a map, but syntactically,
    it doesn't have enclosing `{}`.
- **Map**: a sequence of entries enclosed in `{}`
  - The entries cannot have duplicate keys
- **List**: a sequence of expressions enclosed in `[]`
  - The expressions are called the list's **elements**
