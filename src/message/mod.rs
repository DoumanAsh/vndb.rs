//!Message module

pub mod args;

use ::serde;
//TODO: https://serde.rs/attr-skip-serializing.html
use ::serde_json;

///Constructs string that ca be send to VNDB.
///
///Format: `<command> [arguments] 0x04`
pub fn construct<T: serde::Serialize>(command: &str, arguments: Option<T>) -> String {
    match arguments {
        Some(arg) => format!("{} {}\x04", command, serde_json::to_string(&arg).expect("Couldn't parse argument")),
        None => format!("{}\x04", command)
    }
}
