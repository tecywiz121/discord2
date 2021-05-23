// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Format {
    Png,
    Jpeg,
    WebP,
    Gif,
}

pub trait Image {
    fn supports(&self, format: Format) -> bool;

    fn bare_path(&self) -> &str;

    fn path(&self, format: Format) -> Option<String> {
        if self.supports(format) {
            let ext = match format {
                Format::Png => "png",
                Format::Jpeg => "jpg",
                Format::WebP => "webp",
                Format::Gif => "gif",
            };

            Some(format!("{}.{}", self.bare_path(), ext))
        } else {
            None
        }
    }
}
