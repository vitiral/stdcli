/* Copyright (C) 2017  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

//! `use stdcli::prelude::*;` is considered the standard way to use this library.
//!
//! This contains the primary modules, traits, structs, and functions so that
//! you can get codding in your cli.
//!
//! # Modules
//!
//! - `console`, `dialoguer` : contains the `Term` type and other various types, which allows you to write pretty
//!   things to your console/terminal.
//! - `ctrlc`: the main function is `ctrlc::set_handler`, which allows you to handle ctrlc signals.


// Modules
pub use chrono;
pub use ctrlc;
#[cfg(feature="logging")]
pub use fern;
pub use itertools;
pub use libc;
pub use rayon;  // traits already implemented, this is mostly for `rayon::join` API
pub use regex;  // regex::escape, regex::Error
pub use reqwest; // single request with `reqwest::get`, client connection with `reqwest::Client`
pub use serde;
pub use serde_json;
pub use serde_yaml;
pub use tar;
pub use toml;
pub use walkdir;

// ------------------------------
// -- Traits
pub use itertools::Itertools;
pub use failure::{Fail, ResultExt};
pub use rand::{Rand, Rng, SeedableRng};
pub use rayon::prelude::*;
pub use regex::Replacer;
pub use serde::{Deserialize, Serialize};
pub use shellexpand;
pub use structopt::StructOpt;

// ------------------------------
// -- Structs / Enums
pub use itertools::{Either, EitherOrBoth};
pub use failure::Error;
pub use regex::Regex;
pub use reqwest::Url;
pub use semver::{Version, VersionReq};
pub use tempdir::TempDir;  // the only thing exported by tempdir
pub use walkdir::WalkDir;  // basically the only type of note from walkdir

// ------------------------------
// -- Functions
pub use failure::err_msg;
pub use rand::{random, thread_rng, weak_rng};
#[cfg(feature="logging")]
pub use init_log;
