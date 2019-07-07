//! Contains string related functions.

use crate::{ WriteFn, ReadFn, read, write, BinError };
use std::io::{ Read, Write, Error };

use byteorder::{ ReadBytesExt, WriteBytesExt, BigEndian };

/// Reads/Writes a null terminated ascii string from a stream.
/// 
/// # Remarks
/// The only difference between `null_ascii` and
/// `null_utf8` is the ascii check that fails if the 
/// string is not ascii. Use `null_utf8` instead if you
/// want to avoid that.
/// 
/// # Panics
/// If the input string is not ascii (*NOT* if the string
/// has just been read is not ascii, in that case it will 
/// just return an error) the function panics.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::null_ascii;
/// use bin_io::read;
/// 
/// let vec = vec![ 0x46, 0x6f, 0x6f, 0x00 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, null_ascii())
///     .unwrap();
/// 
/// assert_eq!(string, "Foo");
/// ```
pub fn null_ascii<R: Read, W: Write>() 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (|r: &mut R| {
        let s = read(r, null_utf8())?;

        match s.is_ascii() {
            true => Ok(s),
            false => Err(Error::from(BinError::CheckFail))
        }
    },
    |w: &mut W, s: String| {
        match s.is_ascii() {
            true => write(w, s, null_utf8()),
            false => panic!("String is not ascii")
        }
    })
}

/// Reads/Writes a ascii string from a stream given its length.
/// 
/// # Remarks
/// The only difference between `len_ascii` and
/// `len_utf8` is the ascii check that fails if the 
/// string is not ascii. Use `len_utf8` instead if you
/// want to avoid that.
/// 
/// # Panics
/// If the input string is not ascii (*NOT* if the string
/// has just been read is not ascii, in that case it will 
/// just return an error) the function panics.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::len_ascii;
/// use bin_io::read;
/// 
/// let vec = vec![ 0x42, 0x61, 0x72 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, len_ascii(3))
///     .unwrap();
/// 
/// assert_eq!(string, "Bar");
/// ```
pub fn len_ascii<R: Read, W: Write>(len: usize) 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (move |r: &mut R| {
        let s = read(r, len_utf8(len))?;

        match s.is_ascii() {
            true => Ok(s),
            false => Err(Error::from(BinError::CheckFail))
        }
    },
    move |w: &mut W, s: String| {
        match s.is_ascii() {
            true => write(w, s, len_utf8(len)),
            false => panic!("String is not ascii")
        }
    })
}

/// Reads/Writes a null terminated utf8 string from a stream.
/// 
/// # Remarks
/// The only difference between `null_utf8` and
/// `null_ascii` is the ascii check that fails if the 
/// string is not ascii. Use `null_ascii` instead if you
/// want to check for a strictly ascii string.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::null_utf8;
/// use bin_io::read;
/// 
/// let vec = vec![ 0xf0, 0x9f, 0xa6, 0x80, 0x00 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, null_utf8())
///     .unwrap();
/// 
/// assert_eq!(string, "🦀");
/// ```
pub fn null_utf8<R: Read, W: Write>() 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (|r: &mut R| {
        let mut s = Vec::new();
        loop {
            let c = r.read_u8()?;
            match c {
                0 => break,
                _ => s.push(c)
            }
        }

        String::from_utf8(s)
            .map_err(|e| Error::from(BinError::from(e)))
    },
    |w: &mut W, s: String| {

        w.write_all(&s.as_bytes()[..])
    })
}

/// Reads/Writes an utf8 string from a stream given its length.
/// 
/// # Remarks
/// The only difference between `len_utf8` and
/// `len_ascii` is the ascii check that fails if the 
/// string is not ascii. Use `len_ascii` instead if you
/// want to check for a strictly ascii string.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::len_utf8;
/// use bin_io::read;
/// 
/// let vec = vec![ 0xf0, 0x9f, 0xa6, 0x80 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, len_utf8(4))
///     .unwrap();
/// 
/// assert_eq!(string, "🦀");
/// ```
pub fn len_utf8<R: Read, W: Write>(len: usize) 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (move |r: &mut R| {
        let mut s = vec![0; len];
        r.read_exact(&mut s[..])?;

        String::from_utf8(s)
            .map_err(|e| Error::from(BinError::from(e)))
    },
    move |w: &mut W, s: String| {

        match s.len() == len {
            true => w.write_all(&s.as_bytes()[..]),
            false => panic!("String's length is invalid")
        }
    })
}

/// Reads/Writes a null terminated utf16 string from a stream.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::null_utf16;
/// use bin_io::read;
/// 
/// let vec = vec![ 0xd8, 0x3d, 0xdc, 0x96, 0x00, 0x00 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, null_utf16())
///     .unwrap();
/// 
/// assert_eq!(string, "💖");
/// ```
pub fn null_utf16<R: Read, W: Write>() 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (|r: &mut R| {
        let mut s = Vec::new();
        loop {
            let c = r.read_u16::<BigEndian>()?;
            match c {
                0 => break,
                _ => s.push(c)
            }
        }

        String::from_utf16(&s[..])
            .map_err(|e| Error::from(BinError::from(e)))
    },
    |w: &mut W, s: String| {
        for c in s.encode_utf16() {
            w.write_u16::<BigEndian>(c)?;
        }

        w.write_u16::<BigEndian>(0)
    })
}

/// Reads/Writes an utf16 string from a stream given its length.
/// 
/// # Examples
/// ```
/// # use std::io::Cursor;
/// use bin_io::strings::len_utf16;
/// use bin_io::read;
/// 
/// let vec = vec![ 0xd8, 0x3d, 0xdc, 0x96 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let string = read(&mut cursor, len_utf16(4))
///     .unwrap();
/// 
/// assert_eq!(string, "💖");
/// ```
pub fn len_utf16<R: Read, W: Write>(len: usize) 
-> (impl ReadFn<R, String>, impl WriteFn<W, String>) {

    (move |r: &mut R| {
        let mut s = Vec::new();
        for _ in (0..len).step_by(2) {
            let c = r.read_u16::<BigEndian>()?;
            s.push(c);
        }

        String::from_utf16(&s[..])
            .map_err(|e| Error::from(BinError::from(e)))
    },
    move |w: &mut W, s: String| {
        match s.len() == len {
            true => {
                for c in s.encode_utf16() {
                    w.write_u16::<BigEndian>(c)?;
                }

                Ok(())
            },
            false => panic!("String's length is invalid")
        }

    })
}