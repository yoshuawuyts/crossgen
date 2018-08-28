use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::io;
use std::result;

/// A specialized [`Result`] type for this crate's operations.
///
/// This is generally used to avoid writing out [Error] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
/// [`Error`]: std.struct.Error.html
pub type Result<T> = result::Result<T, Error>;

/// A list enumerating the categories of errors in this crate.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] struct.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug, Fail)]
pub enum ErrorKind {
  /// An error caused by log failure.
  #[fail(display = "An error occurred setting up the logger.")]
  Log,
  /// Any error not part of this list.
  #[fail(display = "Generic error.")]
  Other,
  /// An error caused by Travis
  #[fail(display = "Travis error.")]
  Travis,
  /// An error caused by GitHub
  #[fail(display = "GitHub error.")]
  GitHub,
  /// An error caused by an IO failure.
  #[fail(display = "{}", _0)]
  Io(#[cause] io::Error),
}

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
#[derive(Debug)]
pub struct Error {
  inner: Context<ErrorKind>,
}

impl Error {
  /// Access the [`ErrorKind`] member.
  ///
  /// [`ErrorKind`]: enum.ErrorKind.html
  pub fn kind(&self) -> &ErrorKind {
    &*self.inner.get_context()
  }
}

impl Fail for Error {
  fn cause(&self) -> Option<&Fail> {
    self.inner.cause()
  }

  fn backtrace(&self) -> Option<&Backtrace> {
    self.inner.backtrace()
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.inner, f)
  }
}

impl From<ErrorKind> for Error {
  fn from(kind: ErrorKind) -> Error {
    let inner = Context::new(kind);
    Error { inner }
  }
}

impl From<Context<ErrorKind>> for Error {
  fn from(inner: Context<ErrorKind>) -> Error {
    Error { inner }
  }
}
