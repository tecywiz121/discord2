use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};

use educe::Educe;

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
where
    u64: From<Self>,
{
    fn from_date_time<Tz: TimeZone>(dt: &DateTime<Tz>) -> Option<Self> {
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
    _p: PhantomData<*const For>,
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

unsafe impl<For> Send for Id<For> {}
unsafe impl<For> Sync for Id<For> {}

impl<For> Snowflake for Id<For> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy, Hash)]
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
        let s = TestSnowflake::from_date_time(&expected).unwrap();
        assert_eq!(s.timestamp(), expected);
    }
}
