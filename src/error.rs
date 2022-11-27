use std::{env, fmt, io, result};

use {chrono, nom, thiserror};

/// A common error type for the current crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg_attr(docsrs, doc(hidden))]
    #[error(transparent)]
    ChronoParse(#[from] chrono::ParseError),
    #[cfg_attr(docsrs, doc(hidden))]
    #[error(transparent)]
    EnvVar(#[from] env::VarError),
    #[cfg_attr(docsrs, doc(hidden))]
    #[error(transparent)]
    Fmt(#[from] fmt::Error),
    #[cfg_attr(docsrs, doc(hidden))]
    #[error(transparent)]
    Io(#[from] io::Error),
    #[cfg_attr(docsrs, doc(hidden))]
    #[error(transparent)]
    Nom(#[from] nom::error::VerboseError<String>),
}

/// Type alias for [`Result`](std::result::Result) with an error type of [`Error`].
pub type Result<T> = result::Result<T, Error>;
