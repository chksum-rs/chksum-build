//! Build script required items.

use std::env;
use std::fmt::Write;
use std::str::FromStr;

use chrono::Local;

use crate::cargo::Profile;
use crate::error::Result;
use crate::rust::Toolchain;

/// Wraps [`BuildScript::setup`] to return [`anyhow::Result`] instead of [`Result`].
///
/// # Errors
///
/// Returns an error when [`BuildScript::setup`] will do.
///
/// # Examples
///
/// ```rust,no_run
/// use anyhow::Result;
/// use chksum_build::{setup, BuildScript};
///
/// fn main() -> Result<()> {
///     setup(&BuildScript::default())
/// }
/// ```
pub fn setup(build_script: &BuildScript) -> anyhow::Result<()> {
    build_script.setup()?;
    Ok(())
}

/// Configuration for build script.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BuildScript;

impl BuildScript {
    /// Emits `cargo:*` instructions that set enviroment variables or enable compile-time [`cfg`](https://doc.rust-lang.org/reference/conditional-compilation.html#forms-of-conditional-compilation) settings.
    ///
    /// Resources:
    /// * [The Cargo Book: Environment variables Cargo sets for build scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts),
    /// * [The Cargo Book: Outputs of the Build Script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script).
    ///
    /// # Errors
    ///
    /// Returns an error when environment variables couldn't be parsed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chksum_build::{BuildScript, Result};
    ///
    /// fn main() -> Result<()> {
    ///     BuildScript::default().setup()
    /// }
    /// ```
    pub fn setup(&self) -> Result<()> {
        let mut stdout_buffer = String::new();

        self.setup_build(&mut stdout_buffer)?;

        self.setup_cargo(&mut stdout_buffer)?;

        self.setup_rust(&mut stdout_buffer)?;

        print!("{stdout_buffer}");

        Ok(())
    }

    fn setup_build<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");

        writeln!(stdout, "cargo:rustup-env=CHKSUM_BUILD_INFO_BUILD_DATETIME={datetime}")?;

        Ok(())
    }

    fn setup_cargo<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        let profile = {
            let profile = env::var("PROFILE")?;
            Profile::from_str(&profile)?
        };

        writeln!(stdout, "cargo:rustup-cfg={profile}")?;
        writeln!(stdout, "cargo:rustup-env=CHKSUM_BUILD_INFO_CARGO_PROFILE={profile}")?;

        Ok(())
    }

    fn setup_rust<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        let toolchain = {
            let toolchain = env::var("RUSTUP_TOOLCHAIN")?;
            Toolchain::from_str(&toolchain)?
        };
        let channel = toolchain.channel;

        writeln!(stdout, "cargo:rustup-cfg={channel}")?;
        writeln!(stdout, "cargo:rustup-env=CHKSUM_BUILD_INFO_RUST_CHANNEL={channel}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_build() {
        let mut stdout = String::new();
        assert!(BuildScript::default().setup_build(&mut stdout).is_ok());
        assert_eq!(
            stdout.to_string(),
            format!(
                "cargo:rustup-env=CHKSUM_BUILD_INFO_BUILD_DATETIME={}\n",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            )
        );
    }

    #[test]
    fn test_setup_cargo() {
        env::set_var("PROFILE", "release");

        let mut stdout = String::new();
        assert!(BuildScript::default().setup_cargo(&mut stdout).is_ok());
        assert_eq!(
            stdout.to_string(),
            "cargo:rustup-cfg=release\ncargo:rustup-env=CHKSUM_BUILD_INFO_CARGO_PROFILE=release\n"
        );
    }

    #[test]
    fn test_setup_rust() {
        env::set_var("RUSTUP_TOOLCHAIN", "nightly-x86_64-unknown-linux-gnu");

        let mut stdout = String::new();
        assert!(BuildScript::default().setup_rust(&mut stdout).is_ok());
        assert_eq!(
            stdout.to_string(),
            "cargo:rustup-cfg=nightly\ncargo:rustup-env=CHKSUM_BUILD_INFO_RUST_CHANNEL=nightly\n"
        );
    }
}
