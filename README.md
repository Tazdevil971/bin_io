# bin_io

[![LICENSE](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/bin_io.svg)](https://crates.io/crates/bin_io)

`bin_io` is a crate inspired greatly by `nom` and
other parser combinator libraries.
But `bin_io` differs from those crates since
it aims at providing both reading *and* writing
facilities at the same time, with fewer code.

## Usage
Add `bin_io = "0.1"` to your Cargo.toml

## Big change in 0.2
In 0.2 `bin_io` had a massive change, it now uses 
references while writing, and no longer needs an owned
copy. This meant that some things needed to change
from the last version, but everything should still
work fine (with minor code changes, `seq!` in particular), 
so check out the documentation!

## Example
```rust
use std::io::Cursor;
use bin_io::{ boilerplate, seq, read, write };
use bin_io::numbers::{ be_u8, be_u16 };

#[derive(Clone, Debug, PartialEq, Eq)]
struct Thing {
    a: u8,
    b: u16
}

boilerplate!(
    fn thing_parser() -> Thing {
        seq!(
            Thing { a, b },
            a: be_u8() =>
            b: be_u16() =>
        )
    }
);

let mut vec = Vec::new();
let mut cursor = Cursor::new(vec);

let my_thing = Thing {
    a: 0x10, b: 0x20
};

write(&mut cursor, my_thing.clone(), thing_parser())
    .unwrap();

cursor.set_position(0);

let other_thing = read(&mut cursor, thing_parser())
    .unwrap();

assert_eq!(other_thing, my_thing);
```