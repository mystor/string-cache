// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "string_cache"]
#![crate_type = "rlib"]

#![feature(phase, macro_rules, default_type_params, globs)]
#![no_std]

#[phase(plugin, link)]
extern crate core;

extern crate alloc;
extern crate collections;
extern crate sync;

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate std;

#[phase(plugin)]
extern crate phf_mac;
extern crate phf;

#[phase(plugin)]
extern crate lazy_static;

extern crate xxhash;

#[phase(plugin)]
extern crate string_cache_macros;

#[cfg(feature = "log-events")]
extern crate serialize;

pub use atom::Atom;
pub use namespace::{Namespace, QualName};

#[cfg(feature = "log-events")]
pub mod event;

pub mod atom;
pub mod namespace;

// A private module so that macro-expanded idents like
// `::string_cache::atom::Atom` will also work in this crate.
//
// `libstd` uses the same trick.
#[doc(hidden)]
mod string_cache {
    pub use atom;
    pub use namespace;
}

// For macros and deriving.
#[cfg(not(test))]
mod std {
    pub use core::{cmp, fmt, clone, option, mem, result};
    pub use collections::hash;

    pub mod sync {
        pub use sync::one::{Once, ONCE_INIT};
    }
}
