# bin_io
`bin_io` is a crate inspired greatly by `nom` and
other parser combinator libraries.
But `bin_io` differs from those crates since
it aims at providing both reading *and* writing
facilities at the same time, with fewer code.

## Usage
Add `bin_io = "0.1"` to your Cargo.toml

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