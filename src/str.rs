// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

pub fn obscure<T>(txt: T, f: &mut fmt::Formatter) -> fmt::Result
where
    T: AsRef<str>,
{
    f.write_str(&"*".repeat(txt.as_ref().len()))
}
