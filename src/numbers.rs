//! Contains number related functions.
//! 
//! # Examples
//! ```
//! use std::io::Cursor;
//! use bin_io::numbers::{ le_f32 };
//! use bin_io::{ read, write };
//! 
//! let vec = Vec::new();
//! let mut cursor = Cursor::new(vec);
//! 
//! // Write a Little Endian f32
//! write(&mut cursor, &1.5, le_f32())
//!     .unwrap();
//! 
//! assert_eq!(cursor.get_ref(), &[ 0x00, 0x00, 0xc0, 0x3f ]);
//! 
//! cursor.set_position(0);
//! 
//! // Read a Little Endian f32
//! let val = read(&mut cursor, le_f32())
//!     .unwrap();
//! 
//! assert_eq!(val, 1.5);
//! ```

use crate::{ ReadFn, WriteFn };
use std::io::{ Read, Write };

use byteorder::{ ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian };

macro_rules! auto_impl {
    ($name:ident, $ty:ty, $r:ident, $w:ident, $v:ident, $read:expr, $write:expr) => {
        pub fn $name<R: Read, W: Write>() 
        -> (impl ReadFn<R, $ty>, impl WriteFn<W, $ty>) {
        
            (|$r: &mut R| 
                $read,
            |$w: &mut W, $v: &$ty| 
                $write)
        }
    };
}

auto_impl!(be_u8, u8, r, w, v, r.read_u8(), w.write_u8(*v));
auto_impl!(be_i8, i8, r, w, v, r.read_i8(), w.write_i8(*v));
auto_impl!(le_u8, u8, r, w, v, r.read_u8(), w.write_u8(*v));
auto_impl!(le_i8, i8, r, w, v, r.read_i8(), w.write_i8(*v));

auto_impl!(be_u16, u16, r, w, v, r.read_u16::<BigEndian>(), w.write_u16::<BigEndian>(*v));
auto_impl!(be_i16, i16, r, w, v, r.read_i16::<BigEndian>(), w.write_i16::<BigEndian>(*v));
auto_impl!(le_u16, u16, r, w, v, r.read_u16::<LittleEndian>(), w.write_u16::<LittleEndian>(*v));
auto_impl!(le_i16, i16, r, w, v, r.read_i16::<LittleEndian>(), w.write_i16::<LittleEndian>(*v));

auto_impl!(be_u32, u32, r, w, v, r.read_u32::<BigEndian>(), w.write_u32::<BigEndian>(*v));
auto_impl!(be_i32, i32, r, w, v, r.read_i32::<BigEndian>(), w.write_i32::<BigEndian>(*v));
auto_impl!(le_u32, u32, r, w, v, r.read_u32::<LittleEndian>(), w.write_u32::<LittleEndian>(*v));
auto_impl!(le_i32, i32, r, w, v, r.read_i32::<LittleEndian>(), w.write_i32::<LittleEndian>(*v));

auto_impl!(be_u64, u64, r, w, v, r.read_u64::<BigEndian>(), w.write_u64::<BigEndian>(*v));
auto_impl!(be_i64, i64, r, w, v, r.read_i64::<BigEndian>(), w.write_i64::<BigEndian>(*v));
auto_impl!(le_u64, u64, r, w, v, r.read_u64::<LittleEndian>(), w.write_u64::<LittleEndian>(*v));
auto_impl!(le_i64, i64, r, w, v, r.read_i64::<LittleEndian>(), w.write_i64::<LittleEndian>(*v));

auto_impl!(be_f32, f32, r, w, v, r.read_f32::<BigEndian>(), w.write_f32::<BigEndian>(*v));
auto_impl!(le_f32, f32, r, w, v, r.read_f32::<LittleEndian>(), w.write_f32::<LittleEndian>(*v));

auto_impl!(be_f64, f64, r, w, v, r.read_f64::<BigEndian>(), w.write_f64::<BigEndian>(*v));
auto_impl!(le_f64, f64, r, w, v, r.read_f64::<LittleEndian>(), w.write_f64::<LittleEndian>(*v));