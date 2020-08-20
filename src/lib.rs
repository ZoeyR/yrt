/// Extracts an error from a Result or propogates its value. This macro
/// operates like the inverse of the `try!` macro.
///
/// In case of the `Ok` variant, it retrieves the inner value. `yrt!` then
/// performs conversion using `From`. This provides automatic conversion
/// for general value propogation. The resulting value is then immediately
/// returned.
///
/// Because of the early return, `yrt!` can only be used in functions that
/// return `Result`.
///
/// # Examples
///
/// ```
/// use std::io;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use yrt::yrt;
///
/// // The preferred method of quick returning Errors
/// fn early_return_success() -> Result<File, ()> {
///     let _ = yrt!(File::open("my_best_friends.txt"));
///
///     // The unwrap is just for example code
///     Ok(File::create("my_best_friends.txt").unwrap())
/// }
///
/// // This is equivalent to:
/// fn write_to_file_using_match() -> Result<File, ()> {
///     let _ = match File::open("my_best_friends.txt") {
///         Ok(v) => return Ok(From::from(v)),
///         Err(e) => e,
///     };
///
///     Ok(File::create("my_best_friends.txt").unwrap())
/// }
/// ```
#[macro_export]
macro_rules! yrt {
    ($expr:expr) => {
        match $expr {
            ::std::result::Result::Ok(val) => return ::std::result::Result::Ok(::std::convert::From::from(val)),
            ::std::result::Result::Err(err) => err
        }
    };
    ($expr:expr,) => {
        $crate::yrt!($expr)
    };
}