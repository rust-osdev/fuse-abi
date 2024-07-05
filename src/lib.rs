//! This crate provides bindings to FUSE devices.
//!
//! # Features
//!
//! This crate has the following Cargo features:
//! - `linux` (default) enables the [`linux`] module.
//! - `macos` enables the [`macos`] module.
//! - `num_enum` derives the following traits for enums:
//!   - [`num_enum::IntoPrimitive`]
//!   - [`num_enum::TryFromPrimitive`]
//!   - [`num_enum::UnsafeFromPrimitive`]
//! - `zerocopy` derives the following traits for all structs:
//!   - [`zerocopy::FromZeroes`]
//!   - [`zerocopy::FromBytes`]
//!   - [`zerocopy::AsBytes`]

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(non_camel_case_types)]

/// Linux FUSE device interface ([`fuse(4)`], [`linux/fuse.h`]).
///
/// [`fuse(4)`]: https://www.man7.org/linux/man-pages/man4/fuse.4.html
/// [`linux/fuse.h`]: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/fuse.h?h=v6.9
#[cfg(feature = "linux")]
pub mod linux;

/// macFUSE device interface ([`fuse_kernel.h`]).
///
/// [`fuse_kernel.h`]: https://github.com/osxfuse/fuse/blob/6f7322893456f6ff9db145f096b9bfc2ba95d627/include/fuse_kernel.h
#[cfg(feature = "macos")]
pub mod macos;
