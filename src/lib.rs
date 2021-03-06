// Copyright 2017 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! A stopwatch with lap functionality and nanosecond resolution to time things.
//!
//! Measured times are stored and returned in nanoseconds.
//!
//! # Usage
//!
//! To use `fine_grained`, add the following to the dependencies section of your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fine_grained = "0.1"
//! ```
//!
//! # Examples
//!
//! The `examples` directory contains these examples if you want to actually run them.
//!
//! Get a single measurement:
//!
//! ```
//! extern crate fine_grained;
//!
//! use fine_grained::Stopwatch;
//! #
//! # fn do_something_long() {}
//!
//! fn main() {
//!     // Get a new stopwatch and start it.
//!     let stopwatch = Stopwatch::start_new();
//!
//!     // Do something long and time it.
//!     do_something_long();
//!     println!("Duration: {duration}", duration = stopwatch);
//!     stopwatch.stop();
//! }
//! ```
//!
//! Get measurements for repetitive tasks and a total time:
//!
//! ```
//! extern crate fine_grained;
//!
//! use fine_grained::Stopwatch;
//! #
//! # fn do_something_repetitive() {}
//!
//! fn main() {
//!     // Get a new stopwatch and start it.
//!     let mut stopwatch = Stopwatch::start_new();
//!
//!     // Do something repetitive you want to time.
//!     for _ in 0..10 {
//!         do_something_repetitive();
//!         stopwatch.lap();
//!     }
//!     let stopwatch = stopwatch.stop();
//!
//!     // Print the timing results.
//!     for (i, &lap) in stopwatch.laps().into_iter().enumerate() {
//!         println!("   Round {i}:  {duration}ns", i = i, duration = lap);
//!     }
//!     println!("Total time: {duration}", duration = stopwatch);
//! }
//! ```
//!
//! Get measurements for multiple independent tasks and a total time:
//!
//! ```
//! extern crate fine_grained;
//!
//! use fine_grained::Stopwatch;
//! #
//! # fn do_foo() {}
//! # fn do_bar() {}
//! # fn do_foobar() {}
//!
//! fn main() {
//!     // Get a new stopwatch and start it.
//!     let mut stopwatch = Stopwatch::start_new();
//!
//!     // Do foo.
//!     do_foo();
//!     let time_to_do_foo: u64 = stopwatch.lap();
//!
//!     // Do bar.
//!     do_bar();
//!     let time_to_do_bar: u64 = stopwatch.lap();
//!
//!     // Do foobar.
//!     do_foobar();
//!     let time_to_do_foobar: u64 = stopwatch.lap();
//!
//!     let stopwatch = stopwatch.stop();
//!     println!("Time to do foo: {duration}ns", duration = time_to_do_foo);
//!     println!("Time to do bar: {duration}ns", duration = time_to_do_bar);
//!     println!("Time to do foobar: {duration}ns", duration = time_to_do_foobar);
//!     println!("Total time: {duration}", duration = stopwatch);
//! }
//! ```
//!
//! # Details
//!
//! The `Stopwatch` struct is a state machine with four states: `initialized`, `running`, `paused`, and `stopped`. Since
//! these states are defined on the type level, invalid method calls (e.g. getting a lap from a stopped stopwatch) are
//! recognized during compilation instead of at run time.
//!
//! # Acknowledgements
//!
//! Inspired by Chucky Ellison's stopwatch (https://github.com/ellisonch/rust-stopwatch).

#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]
#![cfg_attr(feature = "cargo-clippy", warn(empty_enum, enum_glob_use, if_not_else, items_after_statements,
                                           missing_docs_in_private_items, nonminimal_bool, option_unwrap_used,
                                           pub_enum_variant_names, print_stdout, result_unwrap_used, similar_names,
                                           single_match_else, stutter, used_underscore_binding, use_debug,
                                           wrong_self_convention, wrong_pub_self_convention))]

extern crate time;

pub use self::stopwatch::Stopwatch;
pub use self::stopwatch::Initialized;
pub use self::stopwatch::Running;
pub use self::stopwatch::Paused;
pub use self::stopwatch::Stopped;

mod stopwatch;
