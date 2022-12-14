//! Cargo related types.

use std::fmt::{self, Display, Formatter};
use std::result;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::error::{context, VerboseError};
use nom::{Finish, IResult};

use crate::error::Error;

/// A Cargo profile.
///
/// Resources:
/// * [The Cargo Book: Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#profiles),
/// * [The Cargo Book: Environment variables Cargo sets for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Profile {
    /// A release build.
    Release,
    /// A non release build.
    Debug,
}

impl Profile {
    const DEBUG_STR: &'static str = "debug";
    const RELEASE_STR: &'static str = "release";

    /// Parse profile.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let release = tag(Self::RELEASE_STR);
        let debug = tag(Self::DEBUG_STR);

        let parser = alt((release, debug));

        let (input, profile) = context("profile", parser)(input)?;

        let profile = match profile {
            Self::RELEASE_STR => Self::Release,
            Self::DEBUG_STR => Self::Debug,
            _ => unreachable!(),
        };

        Ok((input, profile))
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Release => write!(f, "{}", Self::RELEASE_STR),
            Self::Debug => write!(f, "{}", Self::DEBUG_STR),
        }
    }
}

impl FromStr for Profile {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let parser = all_consuming(Self::nom_parse);

        let (_, profile) = context("profile", parser)(s).finish().map_err(|error| {
            let errors = error
                .errors
                .into_iter()
                .map(|(input, kind)| (input.to_string(), kind))
                .collect();
            let error = VerboseError { errors };
            Error::Nom(error)
        })?;

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_profile_display() {
        assert_eq!(format!("{}", Profile::Release), "release");
        assert_eq!(format!("{}", Profile::Debug), "debug");
    }

    #[test]
    fn test_profile_from_str() -> Result<()> {
        assert_eq!(Profile::from_str("release")?, Profile::Release);
        assert!(Profile::from_str("Release").is_err());
        assert!(Profile::from_str("RELEASE").is_err());
        assert_eq!(Profile::from_str("debug")?, Profile::Debug);
        assert!(Profile::from_str("dbg").is_err());
        assert!(Profile::from_str("Debug").is_err());
        assert!(Profile::from_str("DEBUG").is_err());
        Ok(())
    }
}
