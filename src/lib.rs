#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/morc/main/etc/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/refcell/morc/main/etc/favicon.ico",
    issue_tracker_base_url = "https://github.com/refcell/morc/issues/"
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Core markdown types.
pub mod core;
