// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::de::{self, Unexpected, Visitor};

use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub(crate) struct StringOrInteger<T> {
    _p: std::marker::PhantomData<fn() -> T>,
}

impl<'de, T> Visitor<'de> for StringOrInteger<T>
where
    T: FromStr,
    i8: TryInto<T>,
    i16: TryInto<T>,
    i32: TryInto<T>,
    i64: TryInto<T>,
    u8: TryInto<T>,
    u16: TryInto<T>,
    u32: TryInto<T>,
    u64: TryInto<T>,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a positive integer or string")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_str())
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = T::from_str(value)
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))?;

        Ok(value)
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value
            .try_into()
            .map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))?;
        Ok(value)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: T = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value), &self)
        })?;
        Ok(value)
    }
}
