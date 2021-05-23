// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Serialize, Serializer};

use typed_builder::TypedBuilder;

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

#[derive(Debug, Clone, TypedBuilder)]
pub struct UploadImage {
    format: Format,

    #[builder(setter(into))]
    data: Vec<u8>,
}

impl Serialize for UploadImage {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let media_type = match self.format {
            Format::Png => "image/png",
            Format::Jpeg => "image/jpeg",
            Format::Gif => "image/gif",
            Format::WebP => "image/webp",
        };

        let encoded = base64::encode(&self.data);
        let txt = format!("data:{};base64,{}", media_type, encoded);

        txt.serialize(s)
    }
}
