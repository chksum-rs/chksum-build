//! Build information required items.

use chrono::NaiveDateTime;

use crate::cargo::Profile;
use crate::rust::Channel;

/// Creates a [`BuildInfo`] from environment variables.
///
/// # Panics
///
/// Panics when compile time environment variables aren't set.
///
/// # Examples
///
/// ```rust,ignore
/// use chksum_build::build_info;
/// # use chksum_build::Result;
///
/// # fn wrapper() -> Result<()> {
/// let build_info = build_info!();
/// # }
/// ```
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! build_info {
    () => {{
        macro_rules! build {
            () => {{
                let datetime = {
                    let datetime = env!("CHKSUM_BUILD_INFO_BUILD_DATETIME");
                    ::chrono::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S")?
                };

                ::chksum_build::Build::new(datetime)
            }};
        }

        macro_rules! cargo {
            () => {{
                let profile = {
                    use ::std::str::FromStr;

                    let profile = env!("CHKSUM_BUILD_INFO_CARGO_PROFILE");
                    ::chksum_build::cargo::Profile::from_str(profile)?
                };

                ::chksum_build::Cargo::new(profile)
            }};
        }

        macro_rules! rust {
            () => {{
                let channel = {
                    use ::std::str::FromStr;

                    let channel = env!("CHKSUM_BUILD_INFO_RUST_CHANNEL");
                    ::chksum_build::rust::Channel::from_str(channel)?
                };

                ::chksum_build::Rust::new(channel)
            }};
        }

        let build = build!();
        let cargo = cargo!();
        let rust = rust!();

        ::chksum_build::BuildInfo::new(build, cargo, rust)
    }};
}

/// Contains informations about build.
#[derive(Debug, Eq, PartialEq)]
pub struct Build {
    datetime: NaiveDateTime,
}

impl Build {
    #[cfg_attr(docsrs, doc(hidden))]
    #[inline]
    #[must_use]
    pub const fn new(datetime: NaiveDateTime) -> Self {
        Self { datetime }
    }

    /// Returns build datetime.
    #[inline]
    #[must_use]
    pub const fn datetime(&self) -> &NaiveDateTime {
        &self.datetime
    }
}

/// Contains informations about Cargo.
#[derive(Debug, Eq, PartialEq)]
pub struct Cargo {
    profile: Profile,
}

impl Cargo {
    #[cfg_attr(docsrs, doc(hidden))]
    #[inline]
    #[must_use]
    pub const fn new(profile: Profile) -> Self {
        Self { profile }
    }

    /// Returns Cargo profile.
    ///
    /// Check [`Profile`] for more details.
    #[inline]
    #[must_use]
    pub const fn profile(&self) -> &Profile {
        &self.profile
    }
}

/// Contains informations about Rust.
#[derive(Debug, Eq, PartialEq)]
pub struct Rust {
    channel: Channel,
}

impl Rust {
    #[cfg_attr(docsrs, doc(hidden))]
    #[inline]
    #[must_use]
    pub const fn new(channel: Channel) -> Self {
        Self { channel }
    }

    /// Returns Rust channel.
    ///
    /// Check [`Channel`] for more details.
    #[inline]
    #[must_use]
    pub const fn channel(&self) -> &Channel {
        &self.channel
    }
}

/// Contains values set by build script.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Eq, PartialEq)]
pub struct BuildInfo {
    build: Build,
    cargo: Cargo,
    rust: Rust,
}

impl BuildInfo {
    #[cfg_attr(docsrs, doc(hidden))]
    #[inline]
    #[must_use]
    pub const fn new(build: Build, cargo: Cargo, rust: Rust) -> Self {
        Self { build, cargo, rust }
    }

    /// Returns informations about build.
    #[inline]
    #[must_use]
    pub const fn build(&self) -> &Build {
        &self.build
    }

    /// Returns informations about Cargo.
    #[inline]
    #[must_use]
    pub const fn cargo(&self) -> &Cargo {
        &self.cargo
    }

    /// Returns informations about Rust.
    #[inline]
    #[must_use]
    pub const fn rust(&self) -> &Rust {
        &self.rust
    }
}
