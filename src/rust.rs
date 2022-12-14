//! Rust related types.

use std::fmt::{self, Display, Formatter};
use std::result;
use std::str::FromStr;

use chrono::NaiveDate;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{all_consuming, map_res, not, opt, peek, recognize};
use nom::error::{context, VerboseError};
use nom::sequence::{preceded, terminated, tuple};
use nom::{Finish, IResult};

use crate::error::Error;

#[allow(non_camel_case_types)]
#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Architecture {
    // TODO: there are more architectures which are not supported yet
    i686,
    x86_64,
}

impl Architecture {
    const I686_STR: &'static str = "i686";
    const X86_64_STR: &'static str = "x86_64";

    /// Parse architecture.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let i686 = context("i686", tag(Self::I686_STR));
        let x86_64 = context("x86_64", tag(Self::X86_64_STR));

        let mut parser = context("architecture", alt((i686, x86_64)));

        let (input, architecture) = parser(input)?;

        let architecture = match architecture {
            Self::I686_STR => Self::i686,
            Self::X86_64_STR => Self::x86_64,
            _ => unreachable!(),
        };

        Ok((input, architecture))
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::i686 => write!(f, "{}", Self::I686_STR),
            Self::x86_64 => write!(f, "{}", Self::X86_64_STR),
        }
    }
}

/// A rustup channel.
///
/// Resources:
/// * [The rustup book: Channel](https://rust-lang.github.io/rustup/concepts/channels.html),
/// * [The rustup book: Toolchain specification](https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-specification).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Channel {
    /// Stable release.
    Stable,
    /// Beta release that will appear in the next stable release.
    Beta,
    /// Nightly release that is made every night.
    Nightly,
    /// Archive release built with a major and minor version number or a fully specified version number.
    Version(ChannelVersion),
}

impl Channel {
    const BETA_STR: &'static str = "beta";
    const NIGHTLY_STR: &'static str = "nightly";
    const STABLE_STR: &'static str = "stable";

    /// Parse channel.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let mut parser = context("channel", alt((Self::nom_parse_simple, Self::nom_parse_version)));

        parser(input)
    }

    /// Parse simple channel.
    fn nom_parse_simple(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let stable = context("stable", tag(Self::STABLE_STR));
        let beta = context("beta", tag(Self::BETA_STR));
        let nightly = context("nightly", tag(Self::NIGHTLY_STR));

        let mut parser = alt((stable, beta, nightly));

        let (input, channel) = parser(input)?;

        let channel = match channel {
            Self::STABLE_STR => Self::Stable,
            Self::BETA_STR => Self::Beta,
            Self::NIGHTLY_STR => Self::Nightly,
            _ => unreachable!(),
        };

        Ok((input, channel))
    }

    /// Parse version channel.
    fn nom_parse_version(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let mut parser = context("version", ChannelVersion::nom_parse);

        let (input, version) = parser(input)?;

        let channel = Self::Version(version);

        Ok((input, channel))
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "{}", Self::STABLE_STR),
            Self::Beta => write!(f, "{}", Self::BETA_STR),
            Self::Nightly => write!(f, "{}", Self::NIGHTLY_STR),
            Self::Version(version) => write!(f, "{version}"),
        }
    }
}

impl FromStr for Channel {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let mut parser = context("channel", all_consuming(Self::nom_parse));

        let (_, channel) = parser(s).finish().map_err(|error| {
            let errors = error
                .errors
                .into_iter()
                .map(|(input, kind)| (input.to_string(), kind))
                .collect();
            let error = VerboseError { errors };
            Error::Nom(error)
        })?;

        Ok(channel)
    }
}

/// A rustup channel's version.
///
/// Used by [`Channel::Version`] variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelVersion {
    /// A major and minor version number.
    MajorMinor(usize, usize),
    /// A fully specified version number.
    MajorMinorPatch(usize, usize, usize),
}

impl ChannelVersion {
    /// Parse version.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let major_minor = Self::nom_parse_major_minor;
        let major_minor_patch = Self::nom_parse_major_minor_patch;

        let parser = alt((major_minor, major_minor_patch));

        context("version", parser)(input)
    }

    /// Parse (major, minor) version.
    fn nom_parse_major_minor(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let major = context("major", map_res(terminated(digit1, tag(".")), str::parse));
        let minor = context("minor", map_res(digit1, str::parse));
        let not_patch = context("not-patch", peek(not(tag("."))));

        let parser = tuple((major, minor, not_patch));

        let (input, (major, minor, _)) = context("version", parser)(input)?;

        let version = Self::MajorMinor(major, minor);

        Ok((input, version))
    }

    /// Parse (major, minor, patch) version.
    fn nom_parse_major_minor_patch(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let major = context("major", map_res(terminated(digit1, tag(".")), str::parse));
        let minor = context("minor", map_res(terminated(digit1, tag(".")), str::parse));
        let patch = context("patch", map_res(digit1, str::parse));

        let parser = tuple((major, minor, patch));

        let (input, (major, minor, patch)) = context("version", parser)(input)?;

        let version = Self::MajorMinorPatch(major, minor, patch);

        Ok((input, version))
    }
}

impl Display for ChannelVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MajorMinor(major, minor) => write!(f, "{major}.{minor}"),
            Self::MajorMinorPatch(major, minor, patch) => write!(f, "{major}.{minor}.{patch}"),
        }
    }
}

#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Host {
    pub(crate) architecture: Architecture,
    pub(crate) vendor: Option<Vendor>,
    pub(crate) system: System,
}

impl Host {
    /// Parse host.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let architecture = Architecture::nom_parse;
        let vendor = opt(preceded(tag("-"), Vendor::nom_parse));
        let system = preceded(tag("-"), System::nom_parse);

        let parser = tuple((architecture, vendor, system));

        let (input, (architecture, vendor, system)) = context("host", parser)(input)?;

        let host = Self {
            architecture,
            vendor,
            system,
        };

        Ok((input, host))
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            architecture,
            vendor,
            system,
        } = self;

        let vendor = vendor.map_or_else(String::new, |vendor| format!("-{vendor}"));

        write!(f, "{architecture}{vendor}-{system}")
    }
}

impl FromStr for Host {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let mut parser = context("host", all_consuming(Self::nom_parse));

        let (_, host) = parser(s).finish().map_err(|error| {
            let errors = error
                .errors
                .into_iter()
                .map(|(input, kind)| (input.to_string(), kind))
                .collect();
            let error = VerboseError { errors };
            Error::Nom(error)
        })?;

        Ok(host)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinuxAbi {
    // TODO: there are more ABIs which are not supported yet
    GNU,
    GNUX32,
    MUSL,
}

impl LinuxAbi {
    const GNUX32_STR: &'static str = "gnux32";
    const GNU_STR: &'static str = "gnu";
    const MUSL_STR: &'static str = "musl";

    /// Parse ABI.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let gnu = terminated(tag(Self::GNU_STR), not(peek(alpha1)));
        let gnux32 = tag(Self::GNUX32_STR);
        let musl = tag(Self::MUSL_STR);

        let parser = alt((gnu, gnux32, musl));

        let (input, linux_abi) = context("linux-abi", parser)(input)?;

        let linux_abi = match linux_abi {
            Self::GNU_STR => Self::GNU,
            Self::GNUX32_STR => Self::GNUX32,
            Self::MUSL_STR => Self::MUSL,
            _ => unreachable!(),
        };

        Ok((input, linux_abi))
    }
}

impl Display for LinuxAbi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GNU => write!(f, "{}", Self::GNU_STR),
            Self::GNUX32 => write!(f, "{}", Self::GNUX32_STR),
            Self::MUSL => write!(f, "{}", Self::MUSL_STR),
        }
    }
}

#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum System {
    // TODO: there are more systems which are not supported yet
    Darwin,
    Linux(LinuxAbi),
    Windows(WindowsAbi),
}

impl System {
    const DARWIN_STR: &'static str = "darwin";
    const LINUX_STR: &'static str = "linux";
    const WINDOWS_STR: &'static str = "windows";

    /// Parse system.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let darwin = tag(Self::DARWIN_STR);
        let linux = tag(Self::LINUX_STR);
        let windows = tag(Self::WINDOWS_STR);

        let parser = alt((darwin, linux, windows));

        let (input, system) = context("system", parser)(input)?;

        let (input, system) = match system {
            Self::DARWIN_STR => (input, Self::Darwin),
            Self::LINUX_STR => {
                let linux_abi = LinuxAbi::nom_parse;

                let parser = preceded(tag("-"), linux_abi);

                let (input, linux_abi) = context("system-abi", parser)(input)?;

                (input, Self::Linux(linux_abi))
            },
            Self::WINDOWS_STR => {
                let windows_abi = WindowsAbi::nom_parse;

                let parser = preceded(tag("-"), windows_abi);

                let (input, windows_abi) = context("system-abi", parser)(input)?;

                (input, Self::Windows(windows_abi))
            },
            _ => unreachable!(),
        };

        Ok((input, system))
    }
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Darwin => write!(f, "{}", Self::DARWIN_STR),
            Self::Linux(linux_abi) => write!(f, "{}-{linux_abi}", Self::LINUX_STR),
            Self::Windows(windows_abi) => write!(f, "{}-{windows_abi}", Self::WINDOWS_STR),
        }
    }
}

#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Toolchain {
    pub(crate) channel: Channel,
    pub(crate) date: Option<NaiveDate>,
    pub(crate) host: Option<Host>,
}

impl Toolchain {
    /// Parse toolchain.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let channel = Channel::nom_parse;
        let date = opt(preceded(tag("-"), Self::nom_parse_date));
        let host = opt(preceded(tag("-"), Host::nom_parse));

        let parser = tuple((channel, date, host));

        let (input, (channel, date, host)) = context("toolchain", parser)(input)?;

        let toolchain = Self { channel, date, host };

        Ok((input, toolchain))
    }

    /// Parse date.
    fn nom_parse_date(input: &str) -> IResult<&str, NaiveDate, VerboseError<&str>> {
        let year = context("year", digit1);
        let month = context("month", digit1);
        let day = context("day", digit1);

        let parser = recognize(tuple((year, tag("-"), month, tag("-"), day)));

        map_res(context("date", parser), |date| {
            NaiveDate::parse_from_str(date, "%Y-%m-%d")
        })(input)
    }
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { channel, date, host } = self;

        let date = date.map_or_else(String::new, |date| format!("{date}"));
        let host = host.map_or_else(String::new, |host| format!("{host}"));

        write!(f, "{channel}{date}{host}")
    }
}

impl FromStr for Toolchain {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let parser = all_consuming(Self::nom_parse);

        let (_, toolchain) = context("toolchain", parser)(s).finish().map_err(|error| {
            let errors = error
                .errors
                .into_iter()
                .map(|(input, kind)| (input.to_string(), kind))
                .collect();
            let error = VerboseError { errors };
            Error::Nom(error)
        })?;

        Ok(toolchain)
    }
}

#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Vendor {
    // TODO: there are more vendors which are not supported yet
    Apple,
    PC,
    Unknown,
}

impl Vendor {
    const APPLE_STR: &'static str = "apple";
    const PC_STR: &'static str = "pc";
    const UNKNOWN_STR: &'static str = "unknown";

    /// Parse vendor.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let apple = tag(Self::APPLE_STR);
        let pc = tag(Self::PC_STR);
        let unknown = tag(Self::UNKNOWN_STR);

        let parser = alt((apple, pc, unknown));

        let (input, vendor) = context("vendor", parser)(input)?;

        let vendor = match vendor {
            Self::APPLE_STR => Self::Apple,
            Self::PC_STR => Self::PC,
            Self::UNKNOWN_STR => Self::Unknown,
            _ => unreachable!(),
        };

        Ok((input, vendor))
    }
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Apple => write!(f, "{}", Self::APPLE_STR),
            Self::PC => write!(f, "{}", Self::PC_STR),
            Self::Unknown => write!(f, "{}", Self::UNKNOWN_STR),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[cfg_attr(docsrs, doc(hidden))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WindowsAbi {
    // TODO: there are more ABIs which are not supported yet
    GNU,
    GNULLVM,
    MSVC,
}

impl WindowsAbi {
    const GNULLVM_STR: &'static str = "gnullvm";
    const GNU_STR: &'static str = "gnu";
    const MSVC_STR: &'static str = "msvc";

    /// Parse ABI.
    fn nom_parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let gnu = terminated(tag(Self::GNU_STR), not(peek(alpha1)));
        let gnullvm = tag(Self::GNULLVM_STR);
        let msvc = tag(Self::MSVC_STR);

        let parser = alt((gnu, gnullvm, msvc));

        let (input, windows_abi) = context("windows-abi", parser)(input)?;

        let windows_abi = match windows_abi {
            Self::GNU_STR => Self::GNU,
            Self::GNULLVM_STR => Self::GNULLVM,
            Self::MSVC_STR => Self::MSVC,
            _ => unreachable!(),
        };

        Ok((input, windows_abi))
    }
}

impl Display for WindowsAbi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GNU => write!(f, "{}", Self::GNU_STR),
            Self::GNULLVM => write!(f, "{}", Self::GNULLVM_STR),
            Self::MSVC => write!(f, "{}", Self::MSVC_STR),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_channel_display() {
        assert_eq!(format!("{}", Channel::Stable), "stable");
        assert_eq!(format!("{}", Channel::Beta), "beta");
        assert_eq!(format!("{}", Channel::Nightly), "nightly");
        assert_eq!(format!("{}", Channel::Version(ChannelVersion::MajorMinor(1, 3))), "1.3");
        assert_eq!(
            format!("{}", Channel::Version(ChannelVersion::MajorMinorPatch(1, 52, 1))),
            "1.52.1"
        );
    }

    #[test]
    fn test_channel_from_str() -> Result<()> {
        assert_eq!(Channel::from_str("stable")?, Channel::Stable);
        assert!(Channel::from_str("Stable").is_err());
        assert!(Channel::from_str("STABLE").is_err());
        assert_eq!(Channel::from_str("beta")?, Channel::Beta);
        assert!(Channel::from_str("Beta").is_err());
        assert!(Channel::from_str("BETA").is_err());
        assert_eq!(Channel::from_str("nightly")?, Channel::Nightly);
        assert!(Channel::from_str("Nightly").is_err());
        assert!(Channel::from_str("NIGHTLY").is_err());
        assert_eq!(
            Channel::from_str("1.3")?,
            Channel::Version(ChannelVersion::MajorMinor(1, 3))
        );
        assert!(Channel::from_str("1.3.").is_err());
        assert_eq!(
            Channel::from_str("1.52.1")?,
            Channel::Version(ChannelVersion::MajorMinorPatch(1, 52, 1))
        );
        assert!(Channel::from_str("1.52.").is_err());
        assert!(Channel::from_str("1.52.1.").is_err());
        Ok(())
    }

    #[test]
    fn test_host_from_str() -> Result<()> {
        assert_eq!(
            Host::from_str("i686-apple-darwin")?,
            Host {
                architecture: Architecture::i686,
                vendor: Some(Vendor::Apple),
                system: System::Darwin,
            }
        );
        assert_eq!(
            Host::from_str("i686-pc-windows-gnu")?,
            Host {
                architecture: Architecture::i686,
                vendor: Some(Vendor::PC),
                system: System::Windows(WindowsAbi::GNU),
            }
        );
        assert_eq!(
            Host::from_str("i686-pc-windows-msvc")?,
            Host {
                architecture: Architecture::i686,
                vendor: Some(Vendor::PC),
                system: System::Windows(WindowsAbi::MSVC),
            }
        );
        assert_eq!(
            Host::from_str("i686-unknown-linux-gnu")?,
            Host {
                architecture: Architecture::i686,
                vendor: Some(Vendor::Unknown),
                system: System::Linux(LinuxAbi::GNU),
            }
        );
        assert_eq!(
            Host::from_str("i686-unknown-linux-musl")?,
            Host {
                architecture: Architecture::i686,
                vendor: Some(Vendor::Unknown),
                system: System::Linux(LinuxAbi::MUSL),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-apple-darwin")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::Apple),
                system: System::Darwin,
            }
        );
        assert_eq!(
            Host::from_str("x86_64-pc-windows-gnu")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::PC),
                system: System::Windows(WindowsAbi::GNU),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-pc-windows-gnullvm")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::PC),
                system: System::Windows(WindowsAbi::GNULLVM),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-pc-windows-msvc")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::PC),
                system: System::Windows(WindowsAbi::MSVC),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-unknown-linux-gnu")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::Unknown),
                system: System::Linux(LinuxAbi::GNU),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-unknown-linux-gnux32")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::Unknown),
                system: System::Linux(LinuxAbi::GNUX32),
            }
        );
        assert_eq!(
            Host::from_str("x86_64-unknown-linux-musl")?,
            Host {
                architecture: Architecture::x86_64,
                vendor: Some(Vendor::Unknown),
                system: System::Linux(LinuxAbi::MUSL),
            }
        );
        Ok(())
    }

    #[test]
    fn test_toolchain_from_str() -> Result<()> {
        assert_eq!(
            Toolchain::from_str("stable-x86_64-unknown-linux-gnu")?,
            Toolchain {
                channel: Channel::Stable,
                date: None,
                host: Some(Host {
                    architecture: Architecture::x86_64,
                    vendor: Some(Vendor::Unknown),
                    system: System::Linux(LinuxAbi::GNU),
                })
            }
        );
        assert_eq!(
            Toolchain::from_str("nightly-x86_64-unknown-linux-gnu")?,
            Toolchain {
                channel: Channel::Nightly,
                date: None,
                host: Some(Host {
                    architecture: Architecture::x86_64,
                    vendor: Some(Vendor::Unknown),
                    system: System::Linux(LinuxAbi::GNU),
                })
            }
        );
        Ok(())
    }
}
