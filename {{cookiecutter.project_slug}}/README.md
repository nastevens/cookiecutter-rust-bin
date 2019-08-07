# {{ cookiecutter.project_name }}

[![Build Status](https://travis-ci.com/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }}.svg?branch=master)](https://travis-ci.com/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }})
[![Crate](https://img.shields.io/crates/v/{{ cookiecutter.crate_name }}.svg)](https://crates.io/crates/{{ cookiecutter.crate_name }})
[![API](https://docs.rs/{{ cookiecutter.crate_name }}/badge.svg)](https://docs.rs/{{ cookiecutter.crate_name }})
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }}.svg)](http://isitmaintained.com/project/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }})
[![Percentage of issues still open](http://isitmaintained.com/badge/open/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }}.svg)](http://isitmaintained.com/project/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }})
[![Minimum rustc version](https://img.shields.io/badge/rustc-{{ cookiecutter.msrv }}+-lightgray.svg)](https://github.com/{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }}#minimum-supported-rust-version-msrv)

{{ cookiecutter.project_short_description }}

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust {{ cookiecutter.msrv }} and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE)) or
- MIT license ([LICENSE-MIT](/LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
