//! Contains error related definitions.

use std::io::{ Error, ErrorKind };
use quick_error::quick_error;

quick_error! {
    /// Error type used internally by `bin_io`.
    /// 
    /// # Remarks
    /// Keep in mind that for convenience this is always
    /// casted to a `std::io::Error`. So it's unlikely that
    /// you'll ever have to work with it directly.
    #[derive(Debug)]
    pub enum BinError {
        Utf8Conversion(err: std::string::FromUtf8Error) {
            cause(err)
            description("Failed string conversion")
            from()
        }
        Utf16Conversion(err: std::string::FromUtf16Error) {
            cause(err)
            description("Failed string conversion")
            from()
        }
        CheckFail {
            description("Check failed")
        }
        CastFail {
            description("Cast failed")
        }
    }
}

impl From<BinError> for Error {
    fn from(err: BinError) -> Self {
        Self::new(
            ErrorKind::InvalidData,
            err
        )
    }
}