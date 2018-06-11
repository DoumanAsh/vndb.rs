//!Requests toward VNDB.
extern crate serde_json;

use std::fmt;
use std::default::Default;

#[derive(Deserialize, Serialize, Debug, Clone)]
///Login command arguments
///
///Defaults:
///
///* `protocol` - 1;
///* `client` - "rusty";
///* `clientver` - 0.1;
///* `login` - None;
///* `password` - None
pub struct Login {
    ///Protocol. For now should be always 1.
    pub protocol: u8,
    ///Client name
    pub client: &'static str,
    ///Client version
    pub clientver: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///User login
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///User password
    pub password: Option<String>
}

impl Login {
    ///Creates new Login message with provided login/password and other attributes as default.
    pub fn new(login: Option<String>, password: Option<String>) -> Self {
        Login {
            login,
            password,
            ..Login::default()
        }
    }
}

impl fmt::Display for Login {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let payload = serde_json::to_string(&self).expect("Invalid Login message struct");
        write!(f, "login {}", payload)
    }
}

impl Default for Login {
    fn default() -> Self {
        Login {
            protocol: 1,
            client: "rusty",
            clientver: 0.1,
            login: None,
            password: None
        }
    }
}

///Get's command payload.
pub mod get {
    use std::fmt;

    #[derive(Clone)]
    ///Flags for get command.
    ///
    ///Determines which information to retrieve.
    ///Refere to [API](https://vndb.org/d11#5) or [Response module](../response/index.html).
    pub struct Flags {
        inner: Vec<&'static str>
    }

    impl Flags {
        ///Creates new instance with no flags;
        pub fn new() -> Self {
            Self {
                inner: vec![]
            }
        }

        #[inline(always)]
        fn push(mut self, flag: &'static str) -> Self {
            self.inner.push(flag);
            self
        }

        ///Adds basic information.
        pub fn basic(self) -> Self { self.push("basic") }
        ///Adds details information.
        pub fn details(self) -> Self { self.push("details") }
        ///Adds anime information.
        pub fn anime(self) -> Self { self.push("anime") }
        ///Adds relations information.
        pub fn relations(self) -> Self { self.push("relations") }
        ///Adds tags information.
        pub fn tags(self) -> Self { self.push("tags") }
        ///Adds stats information.
        pub fn stats(self) -> Self { self.push("stats") }
        ///Adds screenshots information.
        pub fn screens(self) -> Self { self.push("screens") }
        ///Adds staff information.
        pub fn staff(self) -> Self { self.push("staff") }
        ///Adds vn information.
        pub fn vn(self) -> Self { self.push("vn") }
        ///Adds producers information.
        pub fn producers(self) -> Self { self.push("producers") }
        ///Adds measurement information.
        pub fn meas(self) -> Self { self.push("meas") }
        ///Adds traits information.
        pub fn traits(self) -> Self { self.push("traits") }
        ///Adds vns information.
        pub fn vns(self) -> Self { self.push("vns") }
        ///Adds voiced information.
        pub fn voiced(self) -> Self { self.push("voiced") }
    }

    impl fmt::Display for Flags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.inner.join(","))
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
        pub fn vn() -> Self { Self { inner: "vn" } }
        ///VN's release information.
        pub fn release() -> Self { Self { inner: "release" } }
        ///VN's developer information.
        pub fn producer() -> Self { Self { inner: "producer" } }
        ///VN's character information.
        pub fn character() -> Self { Self { inner: "character" } }
        ///Developer's staff information.
        pub fn staff() -> Self { Self { inner: "staff" } }
        ///User information.
        pub fn user() -> Self { Self { inner: "user" } }
        ///User's votelist.
        pub fn votelist() -> Self { Self { inner: "votelist" } }
        ///User's vnlist.
        pub fn vnlist() -> Self { Self { inner: "vnlist" } }
        ///User's wishlist.
        pub fn wishlist() -> Self { Self { inner: "wishlist" } }

        ///Returns short ID alias of type.
        ///
        ///Can be used in VNDB links as `<short><id>`
        pub fn short<'a>(&'a self) -> &'a str {
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
            write!(f, "({})", self.inner.join(" "))
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    ///Optional Options for get command
    pub struct Options {
        #[serde(skip_serializing_if = "Option::is_none")]
        ///Index of pagination.
        pub page: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ///Number of results per page. Default 10.
        pub results: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ///Name of field to sort by. Default is sort by ID.
        pub sort: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ///Reverse order of results. Default false.
        pub reverse: Option<bool>
    }
}

#[derive(Clone)]
///Get command.
///
///Used to retrieve information about various entities.
pub struct Get {
    ///Type of command. [See](get/Struct.Type.html).
    pub kind: get::Type,
    ///Flags to add. [See](get/Struct.Flags.html)
    pub flags: get::Flags,
    ///Filers. [See](get/Struct.Filters.html)
    pub filters: get::Filters,
    ///Options that control output. [See](get/Struct.Options.html)
    pub options: Option<get::Options>
}

impl fmt::Display for Get {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get {} {} {}", self.kind, self.flags, self.filters)?;

        if let Some(ref options) = self.options {
            let payload = serde_json::to_string(options).expect("Invalid Get message struct");
            write!(f, " {}", payload)
        }
        else {
            Ok(())
        }
    }
}
