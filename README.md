# stdcli: rust meta-library for command line interfaces

This is a rust meta-library. It simply imports multiple good rust libraries
for writing clis and re-exports them.

This is alpha: no code is yet written. I am just compiling a list of crates
to include



- [cargo-script](https://crates.io/crates/cargo-script): this wouldnt be in `stdcli`, but is an important one to know! Quickly write and run cli scripts with crate caching.
- [structopt_derive](https://crates.io/crates/structopt_derive): you already mentioned this one, it's great.
- [tabwriter](https://crates.io/crates/tabwriter) easy formatting of data into a table using `\t` character for alignment
- [self_update](https://crates.io/crates/self_update): auto update/upgrade the compiled binary
- [ansi_term](https://crates.io/crates/ansi_term): colors in the terminal
- [fern](https://crates.io/crates/fern): easier logging for clis
- [fs_extra](https://crates.io/crates/fs_extra): to remove some of your tiny functions
- [ctrlc](https://crates.io/crates/ctrlc): easy handling of unix AND windows signals
- [assert_cli](https://crates.io/crates/assert_cli)
- [indicatif](https://crates.io/crates/indicatif)
- [dialoguer](https://crates.io/crates/dialoguer)
- [console](https://crates.io/crates/console)
- [loggerv](https://crates.io/crates/loggerv): simple logger for logging with colors
- [indoc](https://crates.io/crates/indoc)
