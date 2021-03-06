//! Various utility functions.

use crate::{ WriteFn, ReadFn, BinError };
use std::io::{ Read, Write, Error };
use std::convert::{ TryInto, TryFrom };

/// Binds a value to a writer/reader.
/// 
/// This is an helper function used in conjuction
/// with `seq!`.
/// 
/// ## Reading
/// The function checks the read value agains the
/// input value.
/// 
/// ## Writing
/// The function writes the input value.
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ write, bind, seq };
/// 
/// let vec = Vec::new();
/// let mut cursor = Cursor::new(vec);
/// 
/// let a = seq!(
///     (),
///     bind(be_u8(), 0x50) =>
/// );
/// 
/// write(&mut cursor, &(), a)
///     .unwrap();
/// 
/// assert_eq!(cursor.get_ref()[0], 0x50);
/// ```
pub fn bind<R: Read, W: Write, Rf, Wf, I>(f: (Rf, Wf), i: I) 
-> (impl ReadFn<R, ()>, impl WriteFn<W, ()>)
where Rf: ReadFn<R, I>, Wf: WriteFn<W, I>, I: PartialEq + Clone {

    let (rf, wf) = f;
    let (ri, wi) = (i.clone(), i);

    (move |r: &mut R| {

        match rf(r)?.eq(&ri) {
            true => Ok(()),
            false => Err(Error::from(BinError::CheckFail))
        }
    },
    move |w: &mut W, _v: &()| {

        wf(w, &wi)
    })
}

/// Skips a value to a writer/reader.
/// 
/// This is an helper function used in conjuction
/// with `seq!`.
/// 
/// ## Reading
/// The function just reads the value and then
/// discards the value.
/// 
/// ## Writing
/// The function writes the input value.
/// 
/// # Remarks
/// This function is identical to `bind` with
/// the exception that it won't check the read
/// value.
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ write, skip, seq };
/// 
/// let vec = Vec::new();
/// let mut cursor = Cursor::new(vec);
/// 
/// let a = seq!(
///     (),
///     skip(be_u8(), 0x50) =>
/// );
/// 
/// write(&mut cursor, &(), a)
///     .unwrap();
/// 
/// assert_eq!(cursor.get_ref()[0], 0x50);
/// ```
pub fn skip<R: Read, W: Write, Rf, Wf, I>(f: (Rf, Wf), i: I) 
-> (impl ReadFn<R, ()>, impl WriteFn<W, ()>)
where Rf: ReadFn<R, I>, Wf: WriteFn<W, I>, I: Clone {

    let (rf, wf) = f;

    (move |r: &mut R| {

        rf(r).map(|_| ())
    },
    move |w: &mut W, _v: &()| {

        wf(w, &i)
    })
}

/// Reads/Writes a series of values.
/// 
/// ## Reading
/// The function reads a number of values using
/// the passed parser, the number of values read
/// depends on the input value.
/// 
/// ## Writing
/// The function checks the size of the array against
/// the input value, and panics if there is a mismatch.
/// Then writes the whole array.
/// 
/// # Panics
/// When is writing, the function will only check
/// if the supplied value is the same as the array
/// length, and will panic if there is a mismatch.
/// So remember to initialize everything correctly, 
/// or use a default value (see `seq!` documentation 
/// for details)!
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ write, count, seq };
/// 
/// let vec = Vec::new();
/// let mut cursor = Cursor::new(vec);
/// 
/// let a = count(be_u8(), 3);
/// 
/// write(&mut cursor, &vec![ 10, 20, 30 ], a)
///     .unwrap();
/// 
/// assert_eq!(cursor.get_ref(), &[ 10, 20, 30 ]);
/// ```
pub fn count<R: Read, W: Write, Rf, Wf, I>(f: (Rf, Wf), c: usize)
-> (impl ReadFn<R, Vec<I>>, impl WriteFn<W, Vec<I>>)
where Rf: ReadFn<R, I>, Wf: WriteFn<W, I> {

    let (rf, wf) = f;
    
    (move |r: &mut R| {
        
        let mut vec = Vec::with_capacity(c);
        
        for _ in 0..c {
            vec.push(rf(r)?);
        }

        Ok(vec)
    }, 
    move |w: &mut W, v: &Vec<I>| {
        
        match c == v.len() {
            true => {
                for e in v {
                    wf(w, e)?;
                }

                Ok(())
            },
            false => panic!("Invalid vec length!! Remember to initialize your struct!!")
        }
    })
}

/// Bidirectional cast.
/// 
/// This is an helper function used in conjuction
/// with `seq!`.
/// 
/// ## Reading
/// The function casts O into I.
/// 
/// ## Writing
/// The function casts I into O.
/// 
/// # Remarks
/// Since this cast is bidirectional, each type must
/// be constructible from the other. Therefore remember
/// to implement `From<I> for O` and `From<O> for I`!
pub fn cast<R: Read, W: Write, Rf, Wf, I, O>(f: (Rf, Wf))
-> (impl ReadFn<R, I>, impl WriteFn<W, I>)
where Rf: ReadFn<R, O>, Wf: WriteFn<W, O>, O: From<I> + Into<I>, I: Clone {

    let (rf, wf) = f;
    
    (move |r: &mut R| {
        
        Ok(rf(r)?.into())   
    }, 
    move |w: &mut W, i: &I| {
    
        wf(w, &O::from(i.clone()))
    })
}

/// Reads/Writes an optional value.
/// 
/// This is an helper function used in conjuction
/// with `seq!`.
/// 
/// ## Reading
/// The function reads a value using the passed parser, 
/// the value is read only if the input value is true.
/// 
/// ## Writing
/// The function checks that the `Option` matches the
/// input value and panics otherwise. Then it writes the 
/// value only if present.
/// 
/// # Panics
/// When is writing, the function will only check
/// if the supplied boolean is coherent with the 
/// `Option`, and will panic if there is a mismatch.
/// So remember to initialize everything correctly,
/// or use a default value (see `seq!` documentation 
/// for details)!
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ read, optional, seq };
/// 
/// let vec = vec![ 0 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// # #[derive(Debug, PartialEq, Eq)]
/// struct Unicorn { a: u8, b: Option<u8> };
/// 
/// let a = seq!(
///     Unicorn { a, b },
///     a: be_u8() =>
///     b: optional(
///         be_u8(),
///         a.to_owned() != 0
///     ) =>
/// );
/// 
/// let unicorn = read(&mut cursor, a)
///     .unwrap();
/// 
/// assert_eq!(unicorn, Unicorn { a: 0, b: None });
/// ```
pub fn optional<R: Read, W: Write, Rf, Wf, I>(f: (Rf, Wf), c: bool)
-> (impl ReadFn<R, Option<I>>, impl WriteFn<W, Option<I>>)
where Rf: ReadFn<R, I>, Wf: WriteFn<W, I> {

    let (rf, wf) = f;

    (move |r: &mut R| {
        match c {
            true => Ok(Some(rf(r)?)),
            false => Ok(None)
        }
    },
    move |w: &mut W, i: &Option<I>| {
        match (i, c) {
            (Some(i), true) => wf(w, i),
            (None, false) => Ok(()),
            _ => panic!("Invalid option trigger!! Remember to initialize your struct!!")
        }
    })
}

/// Bidirectional cast.
/// 
/// This is an helper function used in conjuction
/// with `seq!`.
/// 
/// This is the `TryFrom` variant of `cast`.
/// 
/// ## Reading
/// The function tries to casts O into I.
/// 
/// ## Writing
/// The function tries to casts I into O.
/// 
/// # Remarks
/// Since this cast is bidirectional, each type must
/// be constructible from the other. Therefore remember
/// to implement `TryFrom<I> for O` and `TryFrom<O> for I`!
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ write, try_cast, seq };
/// 
/// let vec = Vec::new();
/// let mut cursor = Cursor::new(vec);
/// 
/// struct Unicorn {
///     a: usize
/// }
/// 
/// let a = seq!(
///     Unicorn { a },
///     a: try_cast(be_u8()) =>
/// );
/// 
/// write(&mut cursor, &Unicorn { a: 20 }, a)
///     .unwrap();
/// 
/// assert_eq!(cursor.get_ref()[0], 20);
/// 
/// // Fails to cast!
/// let err = write(&mut cursor, &Unicorn { a: 256 }, a);
/// 
/// assert!(err.is_err());
/// 
/// ```
pub fn try_cast<R: Read, W: Write, Rf, Wf, I, O>(f: (Rf, Wf))
-> (impl ReadFn<R, I>, impl WriteFn<W, I>)
where Rf: ReadFn<R, O>, Wf: WriteFn<W, O>, O: TryFrom<I> + TryInto<I>, I: Clone {

    let (rf, wf) = f;
    
    (move |r: &mut R| {
        
        rf(r)?
            .try_into()
            .map_err(|_| Error::from(BinError::CastFail))
    }, 
    move |w: &mut W, i: &I| {
    
        wf(w, &O::try_from(i.clone())
            .map_err(|_| Error::from(BinError::CastFail))?
        )
    })
}

/// Converts a value to a boolean.
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::numbers::{ be_u8 };
/// use bin_io::{ read, boolean, seq };
/// 
/// let vec = vec![ 0x0 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// struct Thing {
///     a: bool
/// }
/// 
/// let a = seq!(
///     Thing { a },
///     a: boolean(be_u8(), 0x1, 0x0) =>
/// );
/// 
/// let thing = read(&mut cursor, a)
///     .unwrap();
/// 
/// assert_eq!(thing.a, false);
/// ```
pub fn boolean<R: Read, W: Write, Rf, Wf, I>(f: (Rf, Wf), true_val: I, false_val: I)
-> (impl ReadFn<R, bool>, impl WriteFn<W, bool>)
where Rf: ReadFn<R, I>, Wf: WriteFn<W, I>, I: PartialEq + Clone {

    let (rf, wf) = f;
    let (rtrue_val, wtrue_val) = (true_val.clone(), true_val);
    let (rfalse_val, wfalse_val) = (false_val.clone(), false_val);

    (move |r: &mut R| {
        match rf(r)? {
            ref a if a.eq(&rtrue_val) => Ok(true),
            ref a if a.eq(&rfalse_val) => Ok(false),
            _ => Err(Error::from(BinError::CheckFail))
        }
    },
    move |w: &mut W, i: &bool| {
        wf(w, match i {
            true => &wtrue_val,
            false => &wfalse_val
        })
    })
}