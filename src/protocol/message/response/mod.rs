//!VNDB Responses.

use serde::{Serialize, Deserialize};

use core::fmt;
use core::ops::Deref;

pub mod results;
///Typed module for [Results](struct.Results.html)
pub mod typed;

#[derive(Clone, Serialize, Deserialize, Debug)]
///API Error
///
///VNDB API [Reference](https://vndb.org/d11#7)
pub struct VndbError {
    ///Error identifier.
    pub id: String,
    ///Message
    ///
    ///Note that the value of "msg" is not directly linked to the error identifier
    pub msg: String,
}

impl VndbError {
    ///Parses text message into VNDB Error.
    pub fn from_str(error: &str) -> serde_json::Result<Self> {
        serde_json::from_str(error)
    }
}

impl fmt::Display for VndbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error(id='{}')={}", self.id, self.msg)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
///DBstats response
pub struct DBstats {
    ///Number of tags.
    pub tags: u64,
    ///Number of releases.
    pub releases: u64,
    ///Number of producers.
    pub producers: u64,
    ///Number of characters.
    pub chars: u64,
    ///Number of VNs.
    pub vn: u64,
    ///Number of traits.
    pub traits: u64
}

#[derive(Clone, Debug)]
///Loosely typed results of get command.
///
///Due to lack of information on what kind of entity is presented in response,
///the entry point to the response is loosely typed variant.
///
///To get strongly typed variant, use corresponding methods.
///
///Note that most fields are presented, when particular flag specified.
///Due to that most fields, except fiew mandatory, are `Option<T>`.
///
///For convenience purpose, the loosely typed variant implements `Deref` for underlying JSON Value.
pub struct Results {
    inner: serde_json::Value
}

impl Results {
    ///Creates new instance from string with JSON.
    ///
    ///Notes that it expects string to be without special `0x04` character.
    pub fn from_str(results: &str) -> serde_json::Result<Self> {
        Ok(Self {
            inner: serde_json::from_str(results)?
        })
    }

    #[inline(always)]
    fn to<'de, T: Deserialize<'de>>(&'de self) -> serde_json::Result<T> {
        T::deserialize(&self.inner)
    }

    #[inline]
    ///Attempts to convert data to [Vn information](results/Struct.Vn.html).
    pub fn vn(&self) -> serde_json::Result<typed::VN> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [Release information](results/Struct.Release.html).
    pub fn release(&self) -> serde_json::Result<typed::Release> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [Producer information](results/Struct.Producer.html).
    pub fn producer(&self) -> serde_json::Result<typed::Producer> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [Character information](results/Struct.Character.html).
    pub fn character(&self) -> serde_json::Result<typed::Character> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [User information](results/Struct.User.html).
    pub fn user(&self) -> serde_json::Result<typed::User> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [VoteList information](results/Struct.VoteList.html).
    pub fn vote_list(&self) -> serde_json::Result<typed::VoteList> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [VnList information](results/Struct.VnList.html).
    pub fn vn_list(&self) -> serde_json::Result<typed::VnList> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [UList information](results/Struct.UList.html).
    pub fn u_list(&self) -> serde_json::Result<typed::UList> {
        self.to()
    }
}

impl Deref for Results {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
