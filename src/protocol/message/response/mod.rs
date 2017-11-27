//!VNDB Responses.
use ::serde::{Deserialize};
use ::serde_json;

use ::fmt;

pub mod results;

#[derive(Serialize, Deserialize, Debug)]
///API Error
///
///VNDB API [Reference](https://vndb.org/d11#7)
pub struct VndbError {
    pub id: String,
    ///Message
    ///
    ///Note that the value of "msg" is not directly linked to the error identifier
    pub msg: String
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

#[derive(Deserialize, Serialize, Debug)]
///DBstats response
pub struct DBstats {
    pub users: u64,
    pub threads: u64,
    pub tags: u64,
    pub releases: u64,
    pub producers: u64,
    pub chars: u64,
    pub posts: u64,
    pub vn: u64,
    pub traits: u64
}

#[derive(Serialize, Deserialize, Debug)]
///Typed version of `Results`
pub struct TypedResults<T> {
    ///Number of items.
    pub num: u32,
    ///Whether more items is available through pagination.
    pub more: bool,
    ///Underlying entities.
    pub items: Vec<T>
}

///Result of `get vn` command.
pub type ResultsVN = TypedResults<results::Vn>;
///Result of `get release` command.
pub type ResultsRelease = TypedResults<results::Release>;

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
    pub fn vn(&self) -> serde_json::Result<ResultsVN> {
        self.to()
    }

    #[inline]
    ///Attempts to convert data to [Release information](results/Struct.Release.html).
    pub fn release(&self) -> serde_json::Result<ResultsRelease> {
        self.to()
    }
}

use ::ops::Deref;

impl Deref for Results {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
