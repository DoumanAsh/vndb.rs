//! Describes possible arguments

use ::default::Default;

#[derive(Serialize, Debug)]
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
    pub protocol: u32,
    ///Client name
    pub client: String,
    ///Client version
    pub clientver: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///User login
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///User password
    pub password: Option<String>
}

impl Default for Login {
    fn default() -> Self {
        Login {
            protocol: 1,
            client: "rusty".to_string(),
            clientver: 0.1,
            login: None,
            password: None
        }
    }
}
