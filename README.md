# stdcli: batteries included for rust scripts

This is a rust meta-library for including relevant libraries for cli applications. It's
main use case is to be used with [`cargo-script`](https://crates.io/crates/cargo-script).

It is in the same spirit as the [`stdx`](https://github.com/brson/stdx) but serves a
specific instead of general use case.

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

# Licensing

The source code is Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

