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
/// struct Foo {
///     a: u8,
///     b: u16,
///     c: Vec<i32>,
///     d: String,
/// }
/// 
/// let tuple = seq!(
///     // Here we specify wich variables 
///     // are inside Foo
///     Foo { a, b, c, d },
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
#[macro_export]
macro_rules! seq {
    ($ty:ident { $($field:ident),* }, $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl_r $ty {
                $($field),*
            }, r, $($rest)*)
        },
        |w: &mut _, v: _| {
            let $ty {
                $($field),*
            } = v;
            $crate::seq!(__impl_w w, $($rest)*);
            Ok(())
        })
    };

    ((), $($rest:tt)*) => {
        (|r: &mut _| {
            $crate::seq!(__impl_r (), r, $($rest)*)
        },
        |w: &mut _, v: _| {
            $crate::seq!(__impl_w w, $($rest)*);
            Ok(())
        })
    };

    (__impl_r $e:expr, $r:ident, ) => { 
        Ok($e)
    };
    (__impl_r $e:expr, $r:ident, $name:ident : $expr:expr => $($rest:tt)*) => {
        {
            let $name = $crate::read($r, $expr)?;
            $crate::seq!(__impl_r $e, $r, $($rest)*)
        }
    };

    (__impl_r $e:expr, $r:ident, $expr:expr => $($rest:tt)*) => {
        {
            let _: () = $crate::read($r, $expr)?;
            $crate::seq!(__impl_r $e, $r, $($rest)*)
        }
    };

    (__impl_w $w:ident, ) => {};
    (__impl_w $w:ident, $name:ident : $expr:expr => $($rest:tt)*) => {
        {
            $crate::write($w, $name, $expr)?;
            $crate::seq!(__impl_w $w, $($rest)*);
        }
    };

    (__impl_w $w:ident, $expr:expr => $($rest:tt)*) => {
        {
            $crate::write($w, (), $expr)?;
            $crate::seq!(__impl_w $w, $($rest)*);
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