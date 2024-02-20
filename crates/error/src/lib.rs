use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

type Cause = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<ErrorImpl>,
}

struct ErrorImpl {
    kind: ErrorKind,
    cause: Option<Cause>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Reqwest(Reqwest),
    SerdeJson,
    InvalidDifficultyKind,
    ToStr,
    Unit,
}

#[derive(Debug)]
pub enum Reqwest {
    ToStr,
    Default,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub fn new_reqwest_default() -> Error {
        Error::new(ErrorKind::Reqwest(Reqwest::Default))
    }

    pub fn new_reqwest_to_string() -> Error {
        Error::new(ErrorKind::Reqwest(Reqwest::ToStr))
    }

    pub fn new_invalid_difficulty_kind() -> Error {
        Error::new(ErrorKind::InvalidDifficultyKind)
    }

    pub fn new_unit() -> Error {
        Error::new(ErrorKind::Unit)
    }

    pub fn with<C: Into<Cause>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }

    fn description(&self) -> &str {
        match self.inner.kind {
            ErrorKind::Reqwest(Reqwest::Default) => "reqwest error",
            ErrorKind::Reqwest(Reqwest::ToStr) => "reqwest to string error",
            ErrorKind::SerdeJson => "serde_json error",
            ErrorKind::InvalidDifficultyKind => "invalid difficulty kind",
            ErrorKind::ToStr => "to string error",
            ErrorKind::Unit => "unit error",
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple("error::Error");
        f.field(&self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            f.field(cause);
        }
        f.finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.cause.as_ref().map(|cause| &**cause as _)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::new(ErrorKind::Unit)
    }
}

use macros::error_from;

error_from! {
    with_cause reqwest::Error => fn new_reqwest_default;
    with_cause reqwest::header::ToStrError => fn new_reqwest_to_string;
    with_cause serde_json::Error => SerdeJson;
}

mod macros {
    macro_rules! error_from {
        {with_cause $error:path => fn $fn:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::$fn().with(value)
                }
            }

            error_from!{ $($tt)* }
        };
        {with_cause $error:path => $kind:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::new(ErrorKind::$kind).with(value)
                }
            }

            error_from!{ $($tt)* }
        };
        {$error:path => $kind:ident; $($tt:tt)*} => {
            impl From<$error> for Error {
                fn from(_: $error) -> Self {
                    Error::new(ErrorKind::$kind)
                }
            }

            error_from!{ $($tt)* }
        };
        {} => {};
    }

    pub(super) use error_from;
}
