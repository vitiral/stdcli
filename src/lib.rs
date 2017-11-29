//! stdcli: the cli batteries for rust
//!
//! `stdcli` is a metalibrary of the "batteries" of writing cli applications in rust. Its *only
//! primary goal is to make experiementation easier in rust*, with a secondary goal of
//! reducing the use of `version = "*"` in one-off cli scripts so that scripts people write can
//! be more robust.
//!
//! The general intended workflow is:
//! - Use `stdcli` with [`cargo-script`][1] to write single-file scripts that contain all
//!   dependencies
//! - If/when the script grows larger than a single file, break it out using `cargo new` and
//!   use a full-featured project. You can still use `stdcli`.
//! - If/when you need more full control of your dependencies, grab what you need and go!
//!
//! > `stdcli` currently rexports macros so requires a nightly version of rust to be used.
//!
//! [1]: https://github.com/DanielKeep/cargo-script
#![feature(macro_reexport)]
#![allow(unused_imports)]

pub extern crate chrono;
pub extern crate console;
pub extern crate ctrlc;
pub extern crate dialoguer;
#[macro_reexport(format_err)]
#[macro_use]
pub extern crate failure;
#[macro_reexport(Fail)]
#[macro_use] #[no_link]
pub extern crate failure_derive;
#[cfg(feature="logging")]
pub extern crate fern;
pub extern crate indicatif;
#[macro_reexport(iproduct, izip)]
#[macro_use]
pub extern crate itertools;
#[macro_reexport(lazy_static)]
#[macro_use]
pub extern crate lazy_static;
pub extern crate libc;
#[cfg(feature="logging")]
extern crate log;
#[macro_reexport(hashmap, hashset, btreemap, btreeset)]
#[macro_use] #[no_link]
pub extern crate maplit;
pub extern crate rand;
pub extern crate rayon;
pub extern crate regex;
pub extern crate reqwest;
pub extern crate semver;
pub extern crate serde;
#[macro_reexport(Serialize, Deserialize)]
#[macro_use] #[no_link]
pub extern crate serde_derive;
pub extern crate serde_json;
pub extern crate serde_yaml;
pub extern crate shellexpand;
pub extern crate structopt;
#[macro_reexport(StructOpt)]
#[macro_use] #[no_link]
pub extern crate structopt_derive;
pub extern crate tabwriter;
pub extern crate tar;
pub extern crate tempdir;
pub extern crate toml;
pub extern crate walkdir;

pub mod prelude;
pub mod term;


// ------------------------------
// -- Custom Functions

#[cfg(feature="logging")]
/// One liner to init loglevel using a number for the verbosity. The verbosities are:
/// - `0`: exit immediately (no logging)
/// - `1`: Critical + Error
/// - `2`: Info
/// - `3`: Debug
/// - `4`: Trace
///
/// This function *panics* if there is an error. Intended for use in initial script development.
/// Later development should use `fern` or another logging backend directly.
///
/// # Example
/// In your **main** function:
///
/// ```rust
/// #[macro_use]
/// extern crate stdcli;
/// #[macro_use]
/// extern crate log;
///
/// use stdcli::prelude::*;
///
/// fn main() {
///     // verbosity=2, stderr=true, logfile=None
///     init_log(2, true, None);
///     info!("log at the info level");
///     debug!("this will not be logged unless level is increased to 3");
/// }
/// ```
pub fn init_log(
    verbosity: u64,
    stderr: bool,
    logfile: Option<&::std::path::Path>)
{
    use log::LogLevelFilter;
    let level = match verbosity {
        0 => return,
        1 => LogLevelFilter::Error,
        2 => LogLevelFilter::Info,
        3 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    };

    let dispatch = ::fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{: <5} [{} {}] {}",
                record.level(),
                ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                message
            ))
        })
        .level(level);

    let dispatch = if stderr {
        dispatch.chain(std::io::stderr())
    } else {
        dispatch
    };

    let dispatch = if let Some(p) = logfile {
        let f = ::fern::log_file(p)
            .expect(&format!("failed to open logfile: {:?}", p));
        dispatch.chain(f)
    } else {
        dispatch
    };

    dispatch.apply().expect("failed to set logger, was it already initialized?");
}
