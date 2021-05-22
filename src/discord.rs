// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod error;
pub mod requests;

use crate::str::obscure;

use educe::Educe;

use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{ClientBuilder, Response, Url};

pub use self::error::Error;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use snafu::ResultExt;

use std::str::FromStr;

use typed_builder::TypedBuilder;

#[derive(Educe)]
#[educe(Debug)]
enum InnerToken {
    #[educe(Debug(named_field = false))]
    Bot {
        #[educe(Debug(method = "obscure"))]
        bot_token: String,
    },
    #[educe(Debug(named_field = false))]
    Bearer {
        #[educe(Debug(method = "obscure"))]
        bearer_token: String,
    },
}

#[derive(Debug)]
pub struct Token(InnerToken);

impl Token {
    pub fn bot(bot_token: String) -> Self {
        Self(InnerToken::Bot { bot_token })
    }

    pub fn bearer(bearer_token: String) -> Self {
        Self(InnerToken::Bearer { bearer_token })
    }

    fn to_header_value(&self) -> Result<HeaderValue, Error> {
        let (kind, token) = match &self.0 {
            InnerToken::Bot { bot_token } => ("Bot", bot_token),
            InnerToken::Bearer { bearer_token } => ("Bearer", bearer_token),
        };

        let text = format!("{} {}", kind, token);

        let mut value = HeaderValue::from_str(&text)?;
        value.set_sensitive(true);
        Ok(value)
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(doc)]
pub struct Config {
    token: Token,

    #[builder(default_code = "Config::DEFAULT_NAME.to_owned()")]
    name: String,

    #[builder(default_code = "Config::DEFAULT_URL.to_owned()")]
    url: String,

    #[builder(default_code = "Config::DEFAULT_VERSION.to_owned()")]
    version: String,

    #[builder(default_code = "Config::DEFAULT_API_ROOT.to_owned()")]
    api_root: String,
}

impl Config {
    const DEFAULT_NAME: &'static str = "RustDiscord2Bot";
    const DEFAULT_URL: &'static str = env!("CARGO_PKG_REPOSITORY");
    const DEFAULT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const DEFAULT_API_ROOT: &'static str = "https://discord.com/api/v9/";
}

#[derive(Debug, Deserialize)]
struct DiscordError {
    code: Option<u64>,
    message: Option<String>,
}

#[derive(Debug)]
pub struct Discord {
    api_root: Url,
    client: reqwest::Client,
}

impl Discord {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let api_root = Url::from_str(&config.api_root)
            .map_err(|e| Box::new(e) as Box<_>)
            .context(error::InvalidConfig)?;

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, config.token.to_header_value()?);

        let user_agent_txt =
            format!("{} ({}, {})", config.name, config.url, config.version,);
        let user_agent = HeaderValue::from_str(&user_agent_txt)?;

        let client = ClientBuilder::new()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Self { api_root, client })
    }

    fn url<S>(&self, path: S) -> Url
    where
        S: AsRef<str>,
    {
        self.api_root.join(path.as_ref()).unwrap()
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if response.status().is_success() {
            //let json: serde_json::Value = response.json().await?;
            //eprintln!("json: {}", json);
            //Ok(serde_json::from_value(json).unwrap())
            Ok(response.json().await?)
        } else {
            let err: DiscordError = response.json().await?;

            error::Discord {
                code: err.code,
                message: err.message,
            }
            .fail()
        }
    }

    async fn delete<S>(&self, path: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        let url = self.url(path);
        let response = self.client.delete(url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let err: DiscordError = response.json().await?;

            error::Discord {
                code: err.code,
                message: err.message,
            }
            .fail()
        }
    }

    async fn patch<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.patch(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn put<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.put(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn post<S, B, T>(&self, path: S, body: &B) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.url(path);
        let response = self.client.post(url).json(body).send().await?;
        self.handle_response(response).await
    }

    async fn get<S, T>(&self, path: S) -> Result<T, Error>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let url = self.url(path);
        let response = self.client.get(url).send().await?;
        self.handle_response(response).await
    }
}
