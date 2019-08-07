// Copyright (c) {{ cookiecutter.release_date[0:4] }} {{ cookiecutter.full_name }} <{{ cookiecutter.email }}>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod error;

use human_panic::setup_panic;

use crate::error::Error;

fn main() {
    setup_panic!();

    // TODO: everything
    let result = Result::<(), Error>::Ok(());

    match result {
        Ok(()) => ::std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            ::std::process::exit(1);
        }
    }
}
