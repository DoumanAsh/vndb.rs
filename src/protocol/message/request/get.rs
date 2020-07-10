//!Get command parts

use core::fmt;

#[derive(Clone)]
///Flags for get command.
///
///Determines which information to retrieve.
///Refere to [API](https://vndb.org/d11#5) or [Response module](../response/index.html).
pub struct Flags {
    flags: u16,
}

impl Flags {
    const BASIC: u16 = 0b000_000_000_000_01;
    const DETAILS: u16 = 0b000_000_000_000_10;
    const ANIME: u16 = 0b000_000_000_001_00;
    const RELATIONS: u16 = 0b000_000_000_010_00;
    const TAGS: u16 = 0b000_000_000_100_00;
    const STATS: u16 = 0b000_000_001_000_00;
    const SCREENS: u16 = 0b000_000_010_000_00;
    const STAFF: u16 = 0b000_000_100_000_00;
    const VN: u16 = 0b000_001_000_000_00;
    const PRODUCERS: u16 = 0b000_010_000_000_00;
    const MEAS: u16 = 0b000_100_000_000_00;
    const TRAITS: u16 = 0b001_000_000_000_00;
    const VNS: u16 = 0b010_000_000_000_00;
    const VOICED: u16 = 0b100_000_000_000_00;

    ///Creates new instance with no flags;
    pub fn new() -> Self {
        Self {
            flags: 0,
        }
    }

    #[inline(always)]
    fn push(mut self, flag: u16) -> Self {
        self.flags = self.flags | flag;
        self
    }

    ///Adds basic information.
    pub fn basic(self) -> Self { self.push(Self::BASIC) }
    ///Adds details information.
    pub fn details(self) -> Self { self.push(Self::DETAILS) }
    ///Adds anime information.
    pub fn anime(self) -> Self { self.push(Self::ANIME) }
    ///Adds relations information.
    pub fn relations(self) -> Self { self.push(Self::RELATIONS) }
    ///Adds tags information.
    pub fn tags(self) -> Self { self.push(Self::TAGS) }
    ///Adds stats information.
    pub fn stats(self) -> Self { self.push(Self::STATS) }
    ///Adds screenshots information.
    pub fn screens(self) -> Self { self.push(Self::SCREENS) }
    ///Adds staff information.
    pub fn staff(self) -> Self { self.push(Self::STAFF) }
    ///Adds vn information.
    pub fn vn(self) -> Self { self.push(Self::VN) }
    ///Adds producers information.
    pub fn producers(self) -> Self { self.push(Self::PRODUCERS) }
    ///Adds measurement information.
    pub fn meas(self) -> Self { self.push(Self::MEAS) }
    ///Adds traits information.
    pub fn traits(self) -> Self { self.push(Self::TRAITS) }
    ///Adds vns information.
    pub fn vns(self) -> Self { self.push(Self::VNS) }
    ///Adds voiced information.
    pub fn voiced(self) -> Self { self.push(Self::VOICED) }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut has_prev = false;
        const FLAGS: [(u16, &'static str); 14] = [
            (Flags::BASIC, "basic"),
            (Flags::DETAILS, "details"),
            (Flags::ANIME, "anime"),
            (Flags::RELATIONS, "relations"),
            (Flags::TAGS, "tags"),
            (Flags::STATS, "stats"),
            (Flags::SCREENS, "screens"),
            (Flags::STAFF, "staff"),
            (Flags::VN, "vn"),
            (Flags::PRODUCERS, "producers"),
            (Flags::MEAS, "meas"),
            (Flags::TRAITS, "traits"),
            (Flags::VNS, "vns"),
            (Flags::VOICED, "voiced"),
        ];

        for (flag, name) in FLAGS.iter() {
            if self.flags & flag > 0 {
                if has_prev {
                    write!(f, ",")?;
                }

                has_prev = true;
                write!(f, "{}", name)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
///Type of VNDB entity.
///
///On request can be issued only on one type.
///The type determines which [flags](Struct.Flags.html) and [filters](Struct.Filters.html) are available for use.
pub struct Type {
    inner: &'static str
}

impl Type {
    ///VN information.
    pub const fn vn() -> Self { Self { inner: "vn" } }
    ///VN's release information.
    pub const fn release() -> Self { Self { inner: "release" } }
    ///VN's developer information.
    pub const fn producer() -> Self { Self { inner: "producer" } }
    ///VN's character information.
    pub const fn character() -> Self { Self { inner: "character" } }
    ///Developer's staff information.
    pub const fn staff() -> Self { Self { inner: "staff" } }
    ///User information.
    pub const fn user() -> Self { Self { inner: "user" } }
    ///User's votelist.
    pub const fn votelist() -> Self { Self { inner: "votelist" } }
    ///User's vnlist.
    pub const fn vnlist() -> Self { Self { inner: "vnlist" } }
    ///User's wishlist.
    pub const fn wishlist() -> Self { Self { inner: "wishlist" } }
    ///Combination of `votelist`, `vnlist` and `wishlist`.
    pub const fn ulist() -> Self { Self { inner: "ulist" } }

    ///Returns short ID alias of type.
    ///
    ///Can be used in VNDB links as `<short><id>`
    pub fn short(&self) -> &str {
        &self.inner[..1]
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

///Filter expression that produces `format_args`
///
///Example of usage: `filter!(id = 1)`
#[macro_export]
macro_rules! filter {
    ($left:tt $op:tt $var:ident) => {
        format_args!("{} {} {}", stringify!($left), stringify!($op), $var)
    };
    ($left:tt $op:tt $var:tt) => {
        format_args!("{} {} {}", stringify!($left), stringify!($op), stringify!($var))
    }
}

#[derive(Clone)]
///Filters that controls what information to retrieve.
///
///Example of usage:
///
///```
///#[macro_use]
///extern crate vndb;
///
///use vndb::protocol::message::request::get::Filters;
///
///fn main() {
///    assert_eq!(format!("{}", Filters::new().filter(filter!(id = 1)).or(filter!(id = 2))), "(id = 1 or id = 2)");
///}
///```
///
///It produces following expression: `id = 1 or id = 2`.
///
///Macro `filter!()` is available to express simple filters.
///But overall any displayable element is allowed.
pub struct Filters {
    inner: Vec<String>
}

impl Filters {
    ///Creates new instance with no filters.
    pub fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    ///Adds element to filters.
    pub fn filter<T: fmt::Display>(mut self,  element: T) -> Self {
        self.inner.push(format!("{}", element));
        self
    }

    ///Adds new filter with AND condition.
    pub fn and<T: fmt::Display>(self, filter: T) -> Self {
        self.filter("and").filter(filter)
    }

    ///Adds new filter with OR condition.
    pub fn or<T: fmt::Display>(self, filter: T) -> Self {
        self.filter("or").filter(filter)
    }

}

impl fmt::Display for Filters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.len() {
            0 => Ok(()),
            len => {
                let last_element = len - 1;
                write!(f, "(")?;
                for idx in 0..last_element {
                    write!(f, "{} ", unsafe { self.inner.get_unchecked(idx) })?;
                }
                write!(f, "{})", unsafe { self.inner.get_unchecked(last_element) })
            }
        }
    }
}

#[derive(Debug, Clone)]
///Optional Options for get command
pub struct Options<'a> {
    ///Index of pagination.
    pub page: Option<u32>,
    ///Number of results per page. Default 10.
    pub results: Option<u32>,
    ///Name of field to sort by. Default is sort by ID.
    pub sort: Option<&'a str>,
    ///Reverse order of results. Default false.
    pub reverse: Option<bool>
}

impl<'a> fmt::Display for Options<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;

        let mut has_prev = false;
        if let Some(page) = self.page {
            has_prev = true;
            write!(f, "\"page\":{}", page)?;
        }

        if let Some(results) = self.results {
            if has_prev {
                write!(f, ",")?;
            }
            has_prev = true;
            write!(f, "\"results\":{}", results)?;
        }

        if let Some(ref sort) = self.sort {
            if has_prev {
                write!(f, ",")?;
            }
            has_prev = true;
            write!(f, "\"sort\":{}", sort)?;
        }

        if let Some(ref reverse) = self.reverse {
            if has_prev {
                write!(f, ",")?;
            }
            write!(f, "\"reverse\":{}", reverse)?;
        }

        write!(f, "}}")
    }
}
