//! stdcli: the cli batteries for rust

#![feature(macro_reexport)]


extern crate chrono;
extern crate console;
extern crate ctrlc;
extern crate dialoguer;
#[macro_use] extern crate failure;
extern crate fs_extra;
extern crate indicatif;
extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate libc;
#[macro_reexport(log, trace, debug, info, warn, error)]
#[macro_use] #[no_link]
extern crate log;
extern crate loggerv;
#[macro_reexport(hashmap)]
#[macro_use] #[no_link]
extern crate maplit;
extern crate rand;
extern crate rayon;
extern crate regex;
extern crate reqwest;
extern crate semver;
#[macro_use] extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate structopt;
#[macro_use] extern crate structopt_derive;
extern crate tabwriter;
extern crate tar;
extern crate tempdir;
extern crate threadpool;
extern crate toml;
extern crate url;
extern crate walkdir;

