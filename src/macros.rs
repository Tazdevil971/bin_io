/// Macro used to generate a write/read tuple 
/// from sequence of operations.
/// 
/// # Remarks
/// Due to internal limitations assigning a
/// different name to a variable in the capture
/// is not valid:
/// ```compile_fail
/// // Won't compile!!
/// seq!(
///     Bar { a: a1, b: b1 },
///     a1: be_u8 => 
///     b1: be_u8 =>
/// )
/// ```
/// 
/// # Examples
/// ```
/// use std::io::Cursor;
/// use bin_io::{ seq, skip, count, bind, read };
/// use bin_io::numbers::{ be_u8, be_u16, le_u16, be_i32 };
/// use bin_io::strings::null_utf16;
/// 
/// mod bar {
///     pub struct Foo {
///         pub a: u8,
///         pub b: u16,
///         pub c: Vec<i32>,
///         pub d: String,
///     }
/// }
/// 
/// let tuple = seq!(
///     // Here we specify wich variables 
///     // are inside Foo
///     bar::Foo { a, b, c, d },
/// 
///     // And now we start the definition
///     bind(be_u8(), 0x50) =>
///     a: be_u8() =>
///     b: le_u16() =>
///     skip(be_u16(), 1557) =>
///     c: count(be_i32(), b as usize) =>
///     d: null_utf16() =>
/// );
/// 
/// # let mut vec = vec![0; 8];
/// # vec[0] = 0x50;
/// # let mut cursor = Cursor::new(vec);
/// # let r = &mut cursor;
/// 
/// let test = read(r, tuple)
///     .unwrap();
/// ```
/// `seq!` is compatible with multiple data structures 
/// ```
/// use std::io::Cursor;
/// use bin_io::{ seq, read, bind };
/// use bin_io::numbers::{ be_i8, be_i16, be_i32, be_i64 };
/// 
/// mod foo {
///     pub struct Bar1;
///     pub struct Bar2(pub i32);
///     pub struct Bar3 { pub a: i64 }
/// }
/// 
/// let void = seq!(
///     (),
///     bind(be_i8(), -20) =>
/// );
/// 
/// let bar1 = seq!(
///     foo::Bar1,
///     bind(be_i16(), 30) =>
/// );
/// 
/// let bar2 = seq!(
///     foo::Bar2(a),
///     a: be_i32() =>
/// );
/// 
/// let bar3 = seq!(
///     foo::Bar3 { a },
///     a: be_i64() =>
/// );
/// 
/// # let vec = vec![0; 15];
/// # let mut cursor = Cursor::new(vec);
/// # let a = read(&mut cursor, void);
/// # let b = read(&mut cursor, bar1);
/// # let c = read(&mut cursor, bar2);
/// # let d = read(&mut cursor, bar3);
/// ```
/// Sometimes you need extra variables during reading, but you don't
/// want them in your final struct (imagine length/value based formats), 
/// with `seq!` you can do that too!
/// ```
/// use std::io::Cursor;
/// use bin_io::{ seq, read, count };
/// use bin_io::numbers::{ be_u8, be_i16 };
/// 
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo {
///     a: Vec<i16>
/// }
/// 
/// let tuple = seq!(
///     // Capture everything normally
///     Foo { a },
///     // Give the field a default value or some expression to initialize it
///     // Remember: this value is only used during writing and not reading
///     length: be_u8(), a.len() as u8 =>
///     a: count(be_i16(), length as _) =>
/// );
/// 
/// let vec = vec![ 0x2, 0x0, 0x50, 0x0, 0x60 ];
/// let mut cursor = Cursor::new(vec);
/// 
/// let foo = read(&mut cursor, tuple)
///     .unwrap();
/// 
/// assert_eq!(foo, Foo { a: vec![ 0x50, 0x60 ] })
/// ```
#[macro_export]
macro_rules! seq {
    ($($ty:ident)::+ { $($field:ident),* }, $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl r $($ty)::* {
                $($field),*
            }, r, $($rest)*)
        },
        |w: &mut _, v: _| {
            let $($ty)::* {
                $($field),*
            } = v;
            $crate::seq!(__impl w w, $($rest)*);
            Ok(())
        })
    };

    ($($ty:ident)::+ ( $($field:ident),* ), $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl r $($ty)::* (
                $($field),*
            ), r, $($rest)*)
        },
        |w: &mut _, v: _| {
            let $($ty)::* (
                $($field),*
            ) = v;
            $crate::seq!(__impl w w, $($rest)*);
            Ok(())
        })
    };

    ($($ty:ident)::+, $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl r $($ty)::*, r, $($rest)*)
        },
        |w: &mut _, v: _| {
            let $($ty)::* = v;
            $crate::seq!(__impl w w, $($rest)*);
            Ok(())
        })
    };

    ((), $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl r (), r, $($rest)*)
        },
        |w: &mut _, v: _| {
            $crate::seq!(__impl w w, $($rest)*);
            Ok(())
        })
    };

    (__impl r $e:expr, $r:ident, ) => { 
        Ok($e)
    };

    (__impl r $e:expr, $r:ident, $name:ident : $expr:expr => $($rest:tt)*) => {
        {
            let $name = $crate::read($r, $expr)?;
            $crate::seq!(__impl r $e, $r, $($rest)*)
        }
    };
    
    (__impl r $e:expr, $r:ident, $name:ident : $expr:expr, $def:expr => $($rest:tt)*) => {
        {
            let $name = $crate::read($r, $expr)?; 
            $crate::seq!(__impl r $e, $r, $($rest)*)
        }
    };

    (__impl r $e:expr, $r:ident, $expr:expr => $($rest:tt)*) => {
        {
            let _: () = $crate::read($r, $expr)?;
            $crate::seq!(__impl r $e, $r, $($rest)*)
        }
    };

    (__impl w $w:ident, ) => {};

    (__impl w $w:ident, $name:ident : $expr:expr => $($rest:tt)*) => {
        {
            $crate::write($w, $name, $expr)?;
            $crate::seq!(__impl w $w, $($rest)*);
        }
    };

    (__impl w $w:ident, $name:ident : $expr:expr, $def:expr => $($rest:tt)*) => {
        {
            let $name = $def;
            $crate::write($w, $name, $expr)?;
            $crate::seq!(__impl w $w, $($rest)*);
        }
    };

    (__impl w $w:ident, $expr:expr => $($rest:tt)*) => {
        {
            $crate::write($w, (), $expr)?;
            $crate::seq!(__impl w $w, $($rest)*);
        }
    };
}

/// Macro used to remove boilerplate code
/// from a function definition.
/// 
/// # Examples
/// ```
/// use std::io::{ Read, Write };
/// 
/// use bin_io::{ ReadFn, WriteFn };
/// use bin_io::numbers::be_u8;
/// 
/// // Without boilerplate
/// pub fn my_parser_1<R: Read, W: Write>() 
/// -> (impl ReadFn<R, u8>, impl WriteFn<W, u8>) {
///     be_u8()
/// }
/// 
/// // With boilerplate
/// use bin_io::boilerplate;
/// 
/// boilerplate!(
///     pub fn my_parser_2() -> u8 {
///         be_u8()    
///     }
/// );
/// 
/// ```
#[macro_export]
macro_rules! boilerplate {
    ($vis:vis fn $name:ident ( $($arg:ident : $ty:ty),* ) -> $ret:ty { $($tt:tt)* } ) => {
        $vis fn $name <R: std::io::Read, W: std::io::Write> ( $( $arg : $ty )* ) 
        -> (impl $crate::ReadFn<R, $ret>, impl $crate::WriteFn<W, $ret>) {
            $($tt)*
        }
    };
}