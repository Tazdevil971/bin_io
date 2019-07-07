//! # Welcome!
//! `bin_io` is a crate inspired greatly by `nom` and
//! other parser combinator libraries.
//! But `bin_io` differs from those crates since
//! it aims at providing both reading *and* writing
//! facilities at the same time, with fewer code.
//! 
//! ## Example
//! ```
//! use std::io::Cursor;
//! use bin_io::{ boilerplate, seq, read, write };
//! use bin_io::numbers::{ be_u8, be_u16 };
//! 
//! #[derive(Clone, Debug, PartialEq, Eq)]
//! struct Thing {
//!     a: u8,
//!     b: u16
//! }
//! 
//! boilerplate!(
//!     fn thing_parser() -> Thing {
//!         seq!(
//!             Thing { a, b },
//!             a: be_u8() =>
//!             b: be_u16() =>
//!         )
//!     }
//! );
//! 
//! let mut vec = Vec::new();
//! let mut cursor = Cursor::new(vec);
//! 
//! let my_thing = Thing {
//!     a: 0x10, b: 0x20
//! };
//! 
//! write(&mut cursor, my_thing.clone(), thing_parser())
//!     .unwrap();
//! 
//! cursor.set_position(0);
//! 
//! let other_thing = read(&mut cursor, thing_parser())
//!     .unwrap();
//! 
//! assert_eq!(other_thing, my_thing);
//! ```
//! # `nom` or `bin_io`?
//! `bin_io` is at a very early stage of development, so
//! you might want to prefer `nom` over `bin_io` for its
//! well developed stage. But still `bin_io` offers some
//! unique features, and if you fell that it's missing
//! something, I'm open to contrubutions!
//! 
//! # What's happening under the hood?
//! Every function that return a parser (such as `be_u8`)
//! returns a *tuple* of closure, one for reading and one
//! for writing. Every time you apply some other function
//! (for example `bind`) the tuple is split and 
//! a new tuple is created (with each part calling the
//! old closure respectively). Once you call `read` or `write`
//! not only is the correct closure called, but the other
//! type is erased, this is why once you call `read` you
//! can no longer call `write` and viceversa, and you 
//! *always* want to wrap you parser in a function.

pub mod utils;
pub mod error;
#[doc(hidden)]
pub mod macros;
pub mod numbers;
pub mod strings;

pub use utils::*;
pub use error::BinError;

use std::io::{ self, Read, Write };

type ReadDummy = Box<dyn Read>;
type WriteDummy = Box<dyn Write>;

/// Trait representing a read closure.
pub trait ReadFn<R: Read, I>: Fn(&mut R) -> io::Result<I> { }
impl<R: Read, I, F: Fn(&mut R) -> io::Result<I>> ReadFn<R, I> for F { }

/// Trait representing a write closure.
pub trait WriteFn<W: Write, I>: Fn(&mut W, I) -> io::Result<()> { }
impl<W: Write, I, F: Fn(&mut W, I) -> io::Result<()>> WriteFn<W, I> for F { }

/// Reads from a read/write tuple.
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::read;
/// 
/// let vec = vec![ 0x80 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let val = read(&mut cursor, be_u8())
///     .unwrap();
/// 
/// assert_eq!(val, 0x80);
/// ```
pub fn read<R, Rf, Wf, I>(r: &mut R, f: (Rf, Wf)) 
-> io::Result<I>
where R: Read, Rf: ReadFn<R, I>, Wf: WriteFn<WriteDummy, I> {
    f.0(r)
}

/// Writes to a read/write tuple.
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::write;
/// 
/// let vec = Vec::new();
/// let mut cursor = Cursor::new(vec);
/// 
/// let val = write(&mut cursor, 0x80, be_u8())
///     .unwrap();
/// 
/// let vec = cursor.into_inner();
/// assert_eq!(vec[0], 0x80);
/// ```
pub fn write<W, Rf, Wf, I>(w: &mut W, i: I, f: (Rf, Wf))
-> io::Result<()> 
where W: Write, Rf: ReadFn<ReadDummy, I>, Wf: WriteFn<W, I> {
    f.1(w, i)
}