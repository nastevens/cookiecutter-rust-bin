// Copyright (c) {{ cookiecutter.release_date[0:4] }} {{ cookiecutter.full_name }} <{{ cookiecutter.email }}>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    ExampleError,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            ExampleError => write!(f, "ExampleError"),
        }
    }
}

impl StdError for Error {}
