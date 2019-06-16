//!Requests toward VNDB.

use core::fmt;

pub mod get;

#[derive(Debug, Clone)]
///Login command arguments
///
///Defaults:
///
///* `protocol` - 1;
///* `client` - "rusty";
///* `clientver` - 0.1;
///* `creds` - None;
pub struct Login<'a> {
    ///Protocol. For now should be always 1.
    pub protocol: u8,
    ///Client name
    pub client: &'static str,
    ///Client version
    pub clientver: f32,
    ///User credentials
    pub creds: Option<(&'a str, &'a str)>
}

impl<'a> Default for Login<'a> {
    #[inline]
    fn default() -> Self {
        Self::new(None)
    }
}

impl<'a> Login<'a> {
    ///Creates new Login message with provided login/password and other attributes as default.
    pub fn new(creds: Option<(&'a str, &'a str)>) -> Self {
        Login {
            protocol: 1,
            client: "rusty",
            clientver: 0.1,
            creds,
        }
    }
}

impl<'a> fmt::Display for Login<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "login {{\"protocol\":{},\"client\":\"{}\",\"clientver\":{}", self.protocol, self.client, self.clientver)?;
        match self.creds.as_ref() {
            Some((ref login, ref password)) => write!(f, ",\"username\":\"{}\",\"password\":\"{}\"}}", login, password),
            _ => write!(f, "}}"),
        }
    }
}

#[derive(Clone)]
///Get command.
///
///Used to retrieve information about various entities.
pub struct Get<'a> {
    ///Type of command. [See](get/Struct.Type.html).
    pub kind: get::Type,
    ///Flags to add. [See](get/Struct.Flags.html)
    pub flags: get::Flags,
    ///Filers. [See](get/Struct.Filters.html)
    pub filters: get::Filters,
    ///Options that control output. [See](get/Struct.Options.html)
    pub options: Option<get::Options<'a>>
}

impl<'a> fmt::Display for Get<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "get {} {} {}", self.kind, self.flags, self.filters)?;

        match self.options {
            Some(ref options) => {
                write!(f, " {}", options)
            },
            None => Ok(()),
        }
    }
}
