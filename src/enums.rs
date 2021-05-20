// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use snafu::Snafu;

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Snafu, Eq, PartialEq, Clone)]
pub struct EnumFromIntegerError {
    raw: u64,
}

impl EnumFromIntegerError {
    pub(crate) fn new(raw: u64) -> Self {
        Self { raw }
    }

    pub fn inner(self) -> u64 {
        self.raw
    }
}

#[derive(Debug, Snafu, Eq, PartialEq, Clone)]
pub struct ParseEnumError {
    raw: String,
}

impl ParseEnumError {
    pub(crate) fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn into_inner(self) -> String {
        self.raw
    }

    pub fn as_inner(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Clone, Copy)]
enum Inner<T, R> {
    Parsed(T),
    Raw(R),
}

#[derive(Debug, Clone)]
pub struct StringEnum<T>(Inner<T, String>);

impl<T> StringEnum<T> {
    pub fn custom<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(Inner::Raw(s.into()))
    }
}

impl<T> StringEnum<T>
where
    T: Copy,
{
    pub fn unwrap(&self) -> T {
        self.try_unwrap().unwrap()
    }

    pub fn try_unwrap(&self) -> Result<T, ParseEnumError> {
        match &self.0 {
            Inner::Raw(raw) => Err(ParseEnumError { raw: raw.clone() }),
            Inner::Parsed(p) => Ok(p.clone()),
        }
    }
}

impl<T> From<T> for StringEnum<T> {
    fn from(t: T) -> Self {
        Self(Inner::Parsed(t))
    }
}

impl<T> Serialize for StringEnum<T>
where
    T: AsRef<str>,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let txt = match &self.0 {
            Inner::Parsed(t) => t.as_ref(),
            Inner::Raw(s) => s.as_str(),
        };

        txt.serialize(s)
    }
}

impl<'de, T> Deserialize<'de> for StringEnum<T>
where
    T: FromStr,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(d)?;

        let inner = match T::from_str(&raw) {
            Ok(t) => Inner::Parsed(t),
            Err(_) => Inner::Raw(raw),
        };

        Ok(Self(inner))
    }
}

impl<T> fmt::Display for StringEnum<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Inner::Raw(s) => fmt::Display::fmt(s, f),
            Inner::Parsed(p) => fmt::Display::fmt(p, f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntegerEnum<T>(Inner<T, u64>);

impl<T> IntegerEnum<T> {
    pub fn custom(value: u64) -> Self {
        Self(Inner::Raw(value))
    }
}

impl<T> From<T> for IntegerEnum<T> {
    fn from(t: T) -> Self {
        Self(Inner::Parsed(t))
    }
}

impl<T> IntegerEnum<T>
where
    T: Copy,
{
    pub fn unwrap(self) -> T {
        self.try_unwrap().unwrap()
    }

    pub fn try_unwrap(self) -> Result<T, EnumFromIntegerError> {
        match self.0 {
            Inner::Raw(raw) => Err(EnumFromIntegerError { raw }),
            Inner::Parsed(p) => Ok(p),
        }
    }
}

impl<T> Serialize for IntegerEnum<T>
where
    T: Copy + Into<u64>,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let int = match self.0 {
            Inner::Parsed(t) => t.into(),
            Inner::Raw(s) => s,
        };

        int.serialize(s)
    }
}

impl<'de, T> Deserialize<'de> for IntegerEnum<T>
where
    T: TryFrom<u64>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = u64::deserialize(d)?;

        let inner = match T::try_from(raw) {
            Ok(t) => Inner::Parsed(t),
            Err(_) => Inner::Raw(raw.to_owned()),
        };

        Ok(Self(inner))
    }
}

impl<T> From<IntegerEnum<T>> for u64
where
    T: Into<u64>,
{
    fn from(enm: IntegerEnum<T>) -> u64 {
        match enm.0 {
            Inner::Raw(r) => r,
            Inner::Parsed(p) => p.into(),
        }
    }
}
