#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![feature(doc_cfg)]
#![doc = include_str!("../README.md")]

pub mod error;

pub mod auth;
pub use auth::*;

#[cfg(feature = "broker")]
#[doc(cfg(feature = "broker"))]
pub mod discovery;
