//! Tiny library for setting/getting build-time values for your crate.
//!
//! # Setup
//!
//! ## Create `build.rs`
//!
//! Create new file `build.rs` at the top level of your crate (next to `Cargo.toml`).
//!
//! ```rust,no_run
//! use chksum_build::{BuildScript, Result};
//!
//! fn main() -> Result<()> {
//!     BuildScript::default().setup()
//! }
//! ```
//!
//! Optionally you can use along with [`anyhow`].
//!
//! ```rust,no_run
//! use anyhow::Result;
//! use chksum_build::{setup, BuildScript};
//!
//! fn main() -> Result<()> {
//!     setup(&BuildScript::default())
//! }
//! ```
//!
//! ## Update `Cargo.toml`
//!
//! ### Add `package.build` entry
//!
//! ```toml
//! [package]
//! ## ...
//! build = "build.rs"
//! ```
//!
//! ### Add `build-dependencies` entry
//!
//! You can update `Cargo.toml` on your own.
//!
//! ```toml
//! [build-dependencies]
//! ## ...
//! chksum-build = "0.0.1"
//! ```
//!
//! Or use [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand.
//!
//! ```sh
//! cargo add --build chksum-build
//! ```
//!
//! ### Add `dependencies` entry
//!
//! As in the example above you can add entry manually.
//!
//! ```toml
//! [dependencies]
//! ## ...
//! chksum-build = "0.0.1"
//! ```
//!
//! Or by using subcommand.
//!
//! ```sh
//! cargo add chksum-build
//! ```
//!
//! # Usage
//!
//! ## `build_info` macro
//!
//! [`build_info`] macro creates [`BuildInfo`].
//!
//! ```rust,ignore
//! use chksum_build::build_info;
//! # use chksum_build::Result;
//!
//! # fn wrapper() -> Result<()> {
//! let build_info = build_info!();
//! # Ok(())
//! # }
//! ```
//!
//! ## `env` or `option_env` macros
//!
//! [`env`] or [`option_env`] macros.
//!
//! **Notice:** Type conversion need to be done manually.
//!
//! ```rust,ignore
//! use std::str::FromStr;
//! use chksum_build::cargo::Profile;
//!
//! // ...
//!
//! let profile = env!("CHKSUM_BUILD_INFO_CARGO_PROFILE");
//! let profile = Profile::from_str(profile)?;
//!
//! // or
//!
//! let profile = option_env!("CHKSUM_BUILD_INFO_CARGO_PROFILE")
//!                 .map(Profile::from_str)
//!                 .transpose()?;
//! ```
//!
//! ## `cfg` or `cfg_attr` options
//!
//! Some variables are available as configuration options for [`cfg`](https://doc.rust-lang.org/stable/reference/conditional-compilation.html#the-cfg-attribute) or [`cfg_attr`](https://doc.rust-lang.org/stable/reference/conditional-compilation.html#the-cfg_attr-attribute).
//!
//! ### `Profile` variants
//!
//! Check [`Profile`] for more details.
//!
//! ```rust
//! #[cfg(debug)]
//! fn debug_function() {
//!     // ...
//! }
//!
//! #[cfg_attr(release, inline)]
//! fn inline_when_release_function() {
//!     // ...
//! }
//! ```
//!
//! ### `Channel` variants
//!
//! Check [`Channel`] for more details.
//!
//! ```rust
//! #[cfg(stable)]
//! fn stable_function() {
//!     // ...
//! }
//!
//! #[cfg(nightly)]
//! fn nightly_function() {
//!     // ...
//! }
//!
//! #[cfg_attr(nightly, optimize(size))]
//! fn optimize_when_nightly_function() {
//!     // ...
//! }
//! ```
//!
//! # Feature flags
//!
//! * `info`: Enables items required by library or application.
//! * `script`: Enables items required by build script.
//!
//! By default both of them are enabled.
//!
//! # Alternatives
//!
//! * [build-data](https://crates.io/crates/build-data)
//! * [build-info](https://crates.io/crates/build-info)
//! * [built](https://crates.io/crates/built)
//! * [shadow-rs](https://crates.io/crates/shadow-rs)
//! * [vergen](https://crates.io/crates/vergen)
//!
//! # License
//!
//! MIT

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(tarpaulin, feature(no_coverage))]
#![forbid(unsafe_code)]

#[cfg_attr(docsrs, doc(hidden))]
mod cargo;
#[cfg_attr(docsrs, doc(hidden))]
pub mod error;
#[cfg(feature = "info")]
#[cfg_attr(docsrs, doc(cfg(feature = "info")))]
#[cfg_attr(tarpaulin, no_coverage)]
mod info;
#[cfg_attr(docsrs, doc(hidden))]
mod rust;
#[cfg(feature = "script")]
#[cfg_attr(docsrs, doc(cfg(feature = "script")))]
mod script;

pub use cargo::Profile;
pub use error::{Error, Result};
#[cfg(feature = "info")]
pub use info::{Build, BuildInfo, Cargo, Rust};
pub use rust::{Channel, ChannelVersion};
#[cfg(feature = "script")]
pub use script::{setup, BuildScript};
