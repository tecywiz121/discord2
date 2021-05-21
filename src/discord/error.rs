// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use snafu::{Backtrace, IntoError, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(super)")]
#[non_exhaustive]
pub enum Error {
    InvalidConfig {
        source: Box<dyn std::error::Error + 'static>,
        backtrace: Backtrace,
    },

    Reqwest {
        source: Box<dyn std::error::Error + 'static>,
        backtrace: Backtrace,
    },

    Discord {
        code: Option<u64>,
        message: Option<String>,
        backtrace: Backtrace,
    },
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        InvalidConfig {}.into_error(Box::new(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Reqwest {}.into_error(Box::new(err))
    }
}
