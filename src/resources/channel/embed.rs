// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, FixedOffset};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl EmbedFooter {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn proxy_icon_url(&self) -> Option<&str> {
        self.proxy_icon_url.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedImage {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

impl EmbedImage {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }

    pub fn width(&self) -> Option<u64> {
        self.width
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

impl EmbedThumbnail {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }

    pub fn width(&self) -> Option<u64> {
        self.width
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

impl EmbedVideo {
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }

    pub fn width(&self) -> Option<u64> {
        self.width
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

impl EmbedProvider {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

impl EmbedAuthor {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn proxy_icon_url(&self) -> Option<&str> {
        self.proxy_icon_url.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

impl EmbedField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn inline(&self) -> Option<bool> {
        self.inline
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    color: Option<u64>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

impl Embed {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn kind(&self) -> Option<&str> {
        self.kind.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn timestamp(&self) -> Option<DateTime<FixedOffset>> {
        self.timestamp
    }

    pub fn color(&self) -> Option<u64> {
        self.color
    }

    pub fn footer(&self) -> Option<&EmbedFooter> {
        self.footer.as_ref()
    }

    pub fn image(&self) -> Option<&EmbedImage> {
        self.image.as_ref()
    }

    pub fn thumbnail(&self) -> Option<&EmbedThumbnail> {
        self.thumbnail.as_ref()
    }

    pub fn video(&self) -> Option<&EmbedVideo> {
        self.video.as_ref()
    }

    pub fn provider(&self) -> Option<&EmbedProvider> {
        self.provider.as_ref()
    }

    pub fn author(&self) -> Option<&EmbedAuthor> {
        self.author.as_ref()
    }

    pub fn fields(&self) -> Option<&[EmbedField]> {
        self.fields.as_deref()
    }
}
