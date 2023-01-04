# YSCL crash course

Let's get right into it!

```yscl
// Comments begin with 2 slashes and continue to the end of the line.




kantu_version = "1.0.0"
//^^^^^^^^^^^^^^^^^^^^^ This is an entry

// A file is made up of entries.

// Every entry has a _key_ and a _value_

i_am_a_key = "I'm a value!"
//^^^^^^key  ^^^^^^^^^^^^^^value

// The value can be a string, as shown above, but it
// can also be a _map_ or _list_.




// ## Maps

// A map is a sequence of entries enclosed in curly braces (`{}`)
dependencies = {
    foo = "2.0.3"
    bar = "baz"

    lorem = {
        ipsum = "d o l o r"
    }
}




// ## Lists

// A list is a sequence of expressions enclosed in square brackets (`[]`).
licenses = [
    "MIT"
    "APACHE"
    {
        name = "My custom license"
        url = "https://github.com/kylejlin/nonexistent_repo/CUSTOM_LICENSE"
    }
]


// ## String escape sequences

// There are 4 kinds:
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
  // There must be exactly 6 characters (i.e., you cannot omit
  // leading zeroes).
]
```

At this point, you probably get the gist.
For more details, please read [MORE_DETAILS.md](./MORE_DETAILS.md).
