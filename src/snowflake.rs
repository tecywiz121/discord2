// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};

use educe::Educe;

use serde::de::{self, DeserializeOwned, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::cmp::{Eq, Ord};
use std::convert::TryInto;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::str::FromStr;

pub const EPOCH: u64 = 1420070400000;

pub trait Snowflake:
    From<u64>
    + Hash
    + Eq
    + Ord
    + Debug
    + Display
    + Clone
    + Copy
    + FromStr<Err = ParseIntError>
    + Sync
    + Send
    + Serialize
    + DeserializeOwned
where
    u64: From<Self>,
{
    fn from_date_time<Tz: TimeZone>(dt: DateTime<Tz>) -> Option<Self> {
        let unix_ms: u64 = dt.timestamp_millis().try_into().ok()?;
        let discord_ms = unix_ms.checked_sub(EPOCH)?;
        Some(Self::from(discord_ms << 22))
    }

    fn timestamp(self) -> DateTime<Utc> {
        let raw: u64 = self.into();
        let timestamp = (raw >> 22) + EPOCH;
        Utc.timestamp_millis(timestamp.try_into().unwrap())
    }

    fn worker_id(self) -> u8 {
        let raw: u64 = self.into();
        let id = (raw & 0x3E0000) >> 17;

        id as u8
    }

    fn process_id(self) -> u8 {
        let raw: u64 = self.into();
        let id = (raw & 0x1F000) >> 12;

        id as u8
    }

    fn increment(self) -> u16 {
        let raw: u64 = self.into();
        let id = raw & 0xFFF;

        id as u16
    }
}

#[derive(Educe)]
#[educe(
    Debug(named_field = false),
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy
)]
pub struct Id<For> {
    #[educe(Debug(ignore))]
    _p: PhantomData<fn() -> For>,
    id: u64,
}

impl<For> From<Id<For>> for u64 {
    fn from(id: Id<For>) -> Self {
        id.id
    }
}

impl<For> From<u64> for Id<For> {
    fn from(id: u64) -> Self {
        Self {
            _p: PhantomData,
            id,
        }
    }
}

impl<For> Display for Id<For> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<For> FromStr for Id<For> {
    type Err = ParseIntError;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: txt.parse()?,
            _p: PhantomData,
        })
    }
}

impl<For> Serialize for Id<For> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.to_string().serialize(ser)
    }
}

struct StringOrInteger;

impl<'de> Visitor<'de> for StringOrInteger {
    type Value = u64;

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
        let value = u64::from_str(value)
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))?;

        Ok(value)
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Signed(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value
            .try_into()
            .map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))?;
        Ok(value)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value: u64 = value.try_into().map_err(|_| {
            E::invalid_value(Unexpected::Unsigned(value.into()), &self)
        })?;
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }
}

impl<'de, For> Deserialize<'de> for Id<For> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = de.deserialize_any(StringOrInteger)?;

        Ok(Self {
            _p: PhantomData,
            id,
        })
    }
}

impl<For> Snowflake for Id<For> {}

#[derive(Educe)]
#[educe(
    Debug(named_field = false),
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy
)]
pub struct AnyId {
    id: u64,
}

impl From<AnyId> for u64 {
    fn from(id: AnyId) -> Self {
        id.id
    }
}

impl From<u64> for AnyId {
    fn from(id: u64) -> Self {
        Self { id }
    }
}

impl<T> From<Id<T>> for AnyId {
    fn from(id: Id<T>) -> AnyId {
        Self { id: id.id }
    }
}

impl<T> From<AnyId> for Id<T> {
    fn from(id: AnyId) -> Id<T> {
        Self {
            id: id.id,
            _p: PhantomData,
        }
    }
}

impl Display for AnyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl FromStr for AnyId {
    type Err = ParseIntError;

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        Ok(Self { id: txt.parse()? })
    }
}

impl Serialize for AnyId {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.to_string().serialize(ser)
    }
}

impl<'de> Deserialize<'de> for AnyId {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = de.deserialize_any(StringOrInteger)?;

        Ok(Self { id })
    }
}

impl Snowflake for AnyId {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[derive(
        Debug,
        PartialOrd,
        PartialEq,
        Ord,
        Eq,
        Clone,
        Copy,
        Hash,
        Serialize,
        Deserialize,
    )]
    struct TestSnowflake(u64);

    impl Snowflake for TestSnowflake {}

    impl Display for TestSnowflake {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            Display::fmt(&self.0, f)
        }
    }

    impl From<TestSnowflake> for u64 {
        fn from(s: TestSnowflake) -> Self {
            s.0
        }
    }

    impl From<u64> for TestSnowflake {
        fn from(u: u64) -> Self {
            TestSnowflake(u)
        }
    }

    impl FromStr for TestSnowflake {
        type Err = ParseIntError;

        fn from_str(txt: &str) -> Result<Self, Self::Err> {
            Ok(Self(txt.parse()?))
        }
    }

    const EXAMPLE: u64 =
        0b0000001001110001000001100101101011000001000000100000000000000111;

    #[test]
    fn increment() {
        let s = TestSnowflake(EXAMPLE);
        assert_eq!(s.increment(), 0b111);
    }

    #[test]
    fn process_id() {
        let s = TestSnowflake(EXAMPLE);
        assert_eq!(s.process_id(), 0);
    }

    #[test]
    fn worker_id() {
        let s = TestSnowflake(EXAMPLE);
        assert_eq!(s.worker_id(), 1);
    }

    #[test]
    fn timestamp() {
        let s = TestSnowflake(EXAMPLE);
        let expected = Utc.ymd(2016, 4, 30).and_hms_milli(11, 18, 25, 796);
        assert_eq!(s.timestamp(), expected);
    }

    #[test]
    fn from_date_time() {
        let expected = Utc.ymd(2016, 4, 30).and_hms_milli(11, 18, 25, 796);
        let s = TestSnowflake::from_date_time(expected).unwrap();
        assert_eq!(s.timestamp(), expected);
    }

    #[test]
    fn deserialize_string() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Sample {
            id: SampleId,
        }

        type SampleId = Id<Sample>;

        let json = json!({
            "id": "123456799",
        });

        let sample: Sample = serde_json::from_value(json).unwrap();

        assert_eq!(sample.id, SampleId::from(123456799));
    }

    #[test]
    fn deserialize_integer() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Sample {
            id: SampleId,
        }

        type SampleId = Id<Sample>;

        let json = json!({
            "id": 123456799,
        });

        let sample: Sample = serde_json::from_value(json).unwrap();

        assert_eq!(sample.id, SampleId::from(123456799));
    }
}
