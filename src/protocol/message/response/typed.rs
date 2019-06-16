//!Typed VNDB messages

use serde::{Serialize, Deserialize};
use super::{results, Deref};

#[derive(Serialize, Deserialize, Debug)]
///Commont struct of `Results` response.
pub struct Results<T> {
    ///Number of items.
    pub num: u32,
    ///Whether more items is available through pagination.
    pub more: bool,
    ///Underlying entities.
    pub items: Vec<T>
}

    impl<T> Deref for Results<T> {
        type Target = [T];

        fn deref(&self) -> &Self::Target {
            &self.items
        }
    }

///Result of `get vn` command.
pub type VN = Results<results::Vn>;
///Result of `get release` command.
pub type Release = Results<results::Release>;
///Result of `get producer` command.
pub type Producer = Results<results::Producer>;
///Result of `get character` command.
pub type Character = Results<results::Character>;
///Result of `get user` command.
pub type User = Results<results::User>;
///Result of `get votelist` command.
pub type VoteList = Results<results::VoteList>;
///Result of `get vnlist` command.
pub type VnList = Results<results::VnList>;
