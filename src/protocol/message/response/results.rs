//! VNDB entities types.
use serde::{Serialize, Deserialize};
use serde::de::Error;

#[derive(Deserialize, Serialize, Debug)]
///Links for VN.
///
///All fields may be omitted by VNDB.
pub struct VnLinks {
    ///The name of the related article on the Wikipedia
    pub wikipedia: Option<String>,
    ///The URL-encoded tag used on [encubed](http://novelnews.net).
    pub encubed: Option<String>,
    ///The name part of the url on renai.us.
    pub renai: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///Anime related to the VN.
///
///All fields, except `id`, may be omitted by VNDB.
pub struct VnAnime {
    ///[AniDB](http://anidb.net/) ID
    pub id: u64,
    ///[AnimeNewsNetwork](http://animenewsnetwork.com/) ID
    pub ann_id: Option<u64>,
    ///[AnimeNfo](http://animenfo.com/) ID
    pub nfo_id: Option<u64>,
    ///Anime's title in romaji.
    pub title_romaji: Option<String>,
    ///Anime's title in kanji.
    pub title_kanji: Option<String>,
    ///Year in which anime was aired.
    pub year: Option<u16>,
    #[serde(rename = "type")]
    ///Anime's type.
    pub kind: Option<String>
}

///Related VN.
#[derive(Deserialize, Serialize, Debug)]
pub struct VnRelation {
    ///VN's ID.
    pub id: u64,
    ///Description of relation.
    pub relation: String,
    ///Title in romaji.
    pub title: String,
    ///Title in kanji.
    pub original: Option<String>,
    ///Whether it is official.
    pub official: bool
}

#[derive(Deserialize, Serialize, Debug)]
///VN's tag.
pub struct VnTag {
    ///ID.
    pub id: u64,
    ///Score from 0 to 3.
    ///
    ///Note that VNDB's scores are float numbers.
    pub score: f32,
    #[serde(rename = "spoiler level")]
    ///Spoiler severity.
    ///
    ///Possible values:
    ///* 0 - None;
    ///* 1 - Minor;
    ///* 2 - Major;
    pub spoiler: u8
}

#[derive(Deserialize, Serialize, Debug)]
///VN's image.
pub struct VnScreen {
    ///URL.
    pub image: String,
    ///Release's ID.
    pub rid: u64,
    ///Whether it is not safe for work or not.
    pub nsfw: bool,
    ///Image's height.
    pub height: u16,
    ///Image's width.
    pub width: u16,
}

#[derive(Deserialize, Serialize, Debug)]
///VN's staff.
pub struct VnStaff {
    #[serde(rename = "sid")]
    ///Staff's ID.
    pub id: u64,
    #[serde(rename = "aid")]
    ///Staff's Alias ID.
    pub alias: u64,
    ///Name.
    pub name: String,
    ///Name in native language.
    pub original: Option<String>,
    ///Role.
    pub role: String,
    ///Note.
    pub note: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///VN data representation. Returned by `get vn`
pub struct Vn {
    ///Unique identifier of VN.
    pub id: u64,

    //Basic
    ///Main title.
    ///
    ///Provided when `basic` flag is specified.
    pub title: Option<String>,
    ///Title in original language.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub original: Option<String>,
    ///Date of the first release.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub released: Option<String>,
    ///Languages in which VN is available.
    ///
    ///Provided when `basic` flag is specified.
    ///Can be empty array.
    pub languages: Option<Vec<String>>,
    ///Languages of the first release.
    ///
    ///Optionally provided when `basic` flag is specified.
    ///Can be empty array.
    pub orig_lang: Option<Vec<String>>,
    ///Platforms on which VN is available.
    ///
    ///Optionally provided when `basic` flag is specified.
    ///Can be empty array.
    pub platforms: Option<Vec<String>>,

    //Details
    ///Aliases, separated by newline.
    ///
    ///Provided when `details` flag is specified.
    ///Can be `None`.
    pub aliases: Option<String>,
    ///Length of the VN. Between 1-5.
    ///
    ///Provided when `details` flag is specified.
    ///Can be `None`.
    pub length: Option<u8>,
    ///Description of the VN.
    ///
    ///Provided when `details` flag is specified.
    ///Can be `None`.
    pub description: Option<String>,
    ///Links related to the VN.
    ///
    ///Provided when `details` flag is specified.
    pub links: Option<VnLinks>,
    ///HTTP link to VN image.
    ///
    ///Provided when `details` flag is specified.
    ///Can be `None`.
    pub image: Option<String>,
    ///Whether VN's image is NSFW or not.
    ///
    ///Provided when `details` flag is specified.
    pub image_nsfw: Option<bool>,

    //Anime
    ///List of anime related to the VN.
    ///
    ///Provided when `anime` flag is specified.
    pub anime: Option<Vec<VnAnime>>,

    //Relations
    ///List of related VNs.
    ///
    ///Provided when `relations` flag is specified.
    pub relations: Option<Vec<VnRelation>>,

    //Tags
    ///List of VN's tags
    ///
    ///Provided when `tags` flag is specified.
    pub tags: Option<Vec<VnTag>>,

    //Stats
    ///Popularity from 0 to 100.
    ///
    ///Provided when `stats` flag is specified.
    pub popularity: Option<u8>,
    ///VN's rating from 1 to 10.
    ///
    ///Provided when `stats` flag is specified.
    pub rating: Option<f32>,
    ///Number of votes
    ///
    ///Provided when `stats` flag is specified.
    pub votecount: Option<u64>,

    //Screens
    ///List of screenshots
    ///
    ///Provided when `screens` flag is specified.
    pub screens: Option<Vec<VnScreen>>,

    //Staff
    ///List of Staff members.
    ///
    ///Provided when `staff` flag is specified.
    pub staff: Option<Vec<VnStaff>>,
}

#[derive(Deserialize, Serialize, Debug)]
///Type of media for the release.
pub struct ReleaseMedia {
    ///Name.
    pub medium: String,
    ///Quantity.
    #[serde(rename = "qty")]
    pub quantity: Option<u32>
}

#[derive(Deserialize, Serialize, Debug)]
///Related to release VN.
pub struct ReleaseVN {
    ///VN's id.
    pub id: u64,
    ///VN's title.
    pub title: String,
    ///Vn's title in original language.
    pub original: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///Related to release producers.
pub struct ReleaseProducer {
    ///Producer's id.
    pub id: u64,
    ///Whether developer or not.
    pub developer: bool,
    ///Whether publisher or not.
    pub publisher: bool,
    ///Producer's name in romaji.
    pub name: String,
    ///Producer's name in original language.
    pub original: Option<String>,
    #[serde(rename = "type")]
    ///Producer's type
    pub kind: String,
}

#[derive(Deserialize, Serialize, Debug)]
///Release data representation. Returned by `get release`
pub struct Release {
    ///Unique identifier of Release.
    pub id: u64,

    //Basic
    ///Main title.
    ///
    ///Provided when `basic` flag is specified.
    pub title: Option<String>,
    ///Title in original language.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub original: Option<String>,
    ///Date of the first release.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub released: Option<String>,
    #[serde(rename = "type")]
    ///Type of release: "complete", "partial" or "trial".
    ///
    ///Optionally provided when `basic` flag is specified.
    pub kind: Option<String>,
    ///Whether it is a patch or not.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub patch: Option<bool>,
    ///Whether it is a freeware or not.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub freeware: Option<bool>,
    ///Whether it is a doujin or not.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub doujin: Option<bool>,
    ///Languages in which release is available.
    ///
    ///Provided when `basic` flag is specified.
    ///Can be empty array.
    pub languages: Option<Vec<String>>,

    //Details
    ///URL to website.
    ///
    ///Optionally provided when `details` flag is specified.
    pub website: Option<String>,
    ///Some notes.
    ///
    ///Optionally provided when `details` flag is specified.
    pub notes: Option<String>,
    #[serde(rename = "minage")]
    ///Age rating. 0 is all-age.
    ///
    ///Optionally provided when `details` flag is specified.
    pub age: Option<u8>,
    #[serde(rename = "gtin")]
    ///JAN/UPC/EAN code.
    ///
    ///Optionally provided when `details` flag is specified.
    pub code: Option<String>,
    ///Catalogue number.
    ///
    ///Optionally provided when `details` flag is specified.
    pub catalog: Option<String>,
    ///Platforms on which release is available.
    ///
    ///Optionally provided when `details` flag is specified.
    ///Can be empty array.
    pub platforms: Option<Vec<String>>,
    ///Release's media.
    ///
    ///Optionally provided when `details` flag is specified.
    pub media: Option<Vec<ReleaseMedia>>,
    ///Resolution.
    ///
    ///Optionally provided when `details` flag is specified.
    pub resolution: Option<String>,
    ///Voice type available.
    ///
    ///1 = Not voiced, 2 = Only ero scenes voiced, 3 = Partially voiced, 4 = Fully voiced
    ///
    ///Optionally provided when `details` flag is specified.
    pub voiced: Option<u8>,
    ///Animation status.
    ///
    ///Optionally provided when `details` flag is specified.
    pub animation: Option<[u8; 2]>,
    ///Related VNs.
    ///
    ///Optionally provided when `vn` flag is specified.
    pub vn: Option<Vec<ReleaseVN>>,
    ///Related producers.
    ///
    ///Optionally provided when `producer` flag is specified.
    pub producers: Option<Vec<ReleaseProducer>>,
}

#[derive(Deserialize, Serialize, Debug)]
///External links related for [Producer](struct.Prodcer.html)
pub struct ProducerLinks {
    ///Official homepage.
    pub homepage: Option<String>,
    ///Title of english wikipedia's article.
    pub wikipedia: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///External links related for [Producer](struct.Prodcer.html)
pub struct ProducerRelation {
    ///Unique identifier of Producer.
    pub id: u64,
    ///Relation to [Producer](struct.Prodcer.html).
    pub relation: String,
    ///Name(romaji).
    pub name: String,
    ///Name in original language.
    pub original: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///Producer data representation. Returned by `get producer`
pub struct Producer {
    ///Unique identifier of Producer.
    pub id: u64,

    //Basic
    ///Name(romaji).
    ///
    ///Provided when `basic` flag is specified.
    pub name: Option<String>,
    ///Name in original language.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub original: Option<String>,
    #[serde(rename = "type")]
    ///Type.
    ///
    ///Provided when `basic` flag is specified.
    pub kind: Option<String>,
    ///Primary language.
    ///
    ///Provided when `basic` flag is specified.
    pub language: Option<String>,

    //Details
    ///Related links.
    ///
    ///Provided when `details` flag is specified.
    pub links: Option<ProducerLinks>,
    ///Aliases, separated by newline.
    ///
    ///Optionally provided when `details` flag is specified.
    pub aliases: Option<String>,
    ///Description/notes on producer.
    ///
    ///Optionally provided when `details` flag is specified.
    pub description: Option<String>,

    //Relations
    ///List of related producers.
    ///
    ///Provided when `relations` flag is specified.
    pub relations: Option<Vec<ProducerRelation>>,
}

#[derive(Debug)]
///[Character](struct.Character.html)'s gender
pub enum CharacterGender {
    ///Male gender.
    Male,
    ///Female gender.
    Female,
    ///Both genders?
    Both
}

impl<'de> Deserialize<'de> for CharacterGender {
    fn deserialize<D: serde::de::Deserializer<'de>>(gender: D) -> Result<Self, D::Error> {
        let gender: &'de str = Deserialize::deserialize(gender)?;
        match gender {
            "m" => Ok(CharacterGender::Male),
            "f" => Ok(CharacterGender::Female),
            "b" => Ok(CharacterGender::Both),
            _ => Err(D::Error::custom(format!("Unknown type '{}' of character gender.", gender)))
        }
    }
}

impl Serialize for CharacterGender {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            CharacterGender::Male => serializer.serialize_str("m"),
            CharacterGender::Female => serializer.serialize_str("f"),
            CharacterGender::Both => serializer.serialize_str("b")
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
///Character's Voice Actress.
pub struct CharacterSeiyuu {
    ///Unique identifier of staff.
    pub id: u64,
    ///Alias identifier of staff.
    pub aid: u64,
    ///VN identifier.
    pub vid: u64,
    ///Note
    pub note: String,
}

#[derive(Deserialize, Serialize, Debug)]
///Character data representation. Returned by `get character`
pub struct Character {
    ///Unique identifier of Character.
    pub id: u64,

    //Basic
    ///Name(romaji).
    ///
    ///Provided when `basic` flag is specified.
    pub name: Option<String>,
    ///Name in original language.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub original: Option<String>,
    ///Character's gender.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub gender: Option<CharacterGender>,
    ///Blood type.
    ///
    ///Optionally provided when `basic` flag is specified.
    #[serde(rename = "bloodt")]
    pub blood_type: Option<String>,
    ///Birthday as tuple `(day, month)`
    ///
    ///Provided when `basic` flag is specified.
    pub birthday: Option<(u8, u8)>,

    //Details
    ///Aliases, separated by newline.
    ///
    ///Provided when `details` flag is specified.
    pub aliases: Option<String>,
    ///Description/notes.
    ///
    ///Optionally provided when `details` flag is specified.
    pub description: Option<String>,
    ///URL to image.
    ///
    ///Optionally provided when `details` flag is specified.
    pub image: Option<String>,

    //Meas
    ///Bust in cm.
    ///
    ///Optionally provided when `meas` flag is specified.
    pub bust: Option<u16>,
    ///Waist in cm.
    ///
    ///Optionally provided when `meas` flag is specified.
    pub waist: Option<u16>,
    ///Hip in cm.
    ///
    ///Optionally provided when `meas` flag is specified.
    pub hip: Option<u16>,
    ///Height in cm.
    ///
    ///Optionally provided when `meas` flag is specified.
    pub height: Option<u16>,
    ///Weight in kg.
    ///
    ///Optionally provided when `meas` flag is specified.
    pub weight: Option<u16>,

    //Traits
    ///List, possibly empty, of traits specified as tuple `(id, spoiler level)`.
    ///
    ///Provided when `traits` flag is specified.
    pub traits: Option<Vec<(u64, u8)>>,

    //vns
    ///List, possibly empty, of related VNs specified as tuple `(vn id, release id, spoiler level, role)`.
    ///
    ///Provided when `vns` flag is specified.
    pub vns: Option<Vec<(u64, u64, u8, String)>>,

    //voiced
    ///List, possibly empty, of related VNs specified as tuple `(vn id, release id, spoiler level, role)`.
    ///
    ///Provided when `voiced` flag is specified.
    pub voiced: Option<Vec<CharacterSeiyuu>>
}

#[derive(Deserialize, Serialize, Debug)]
///User data representation. Returned by `get user`
pub struct User {
    ///Unique identifier of User.
    ///
    ///Provided when `basic` flag is specified.
    pub id: Option<u64>,
    ///User's name.
    ///
    ///Provided when `basic` flag is specified.
    #[serde(rename = "username")]
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
///Vote list data representation. Returned by `get user`
pub struct VoteList {
    ///Unique identifier of User.
    ///
    ///Provided when `basic` flag is specified.
    pub uid: Option<u64>,
    ///Unique identifier of VN.
    ///
    ///Provided when `basic` flag is specified.
    pub vn: Option<u64>,
    ///Vote value in range from 10 to 100.
    ///
    ///Provided when `basic` flag is specified.
    pub vote: Option<u8>,
    ///Unix timestamp of when this vote is added.
    ///
    ///Provided when `basic` flag is specified.
    pub added: Option<u64>,
}

#[derive(Copy, Clone, Debug)]
///Status in [VnList](struct.VnList.html).
pub enum VnStatus {
    ///Unknown.
    Unknown = 0,
    ///Currently playing.
    Playing = 1,
    ///Finished.
    Finished = 2,
    ///Stalled.
    Stalled = 3,
    ///Dropped.
    Dropped = 4
}

impl<'de> Deserialize<'de> for VnStatus {
    fn deserialize<D: serde::de::Deserializer<'de>>(status: D) -> Result<Self, D::Error> {
        let status: u8 = Deserialize::deserialize(status)?;
        match status {
            0 => Ok(VnStatus::Unknown),
            1 => Ok(VnStatus::Playing),
            2 => Ok(VnStatus::Finished),
            3 => Ok(VnStatus::Stalled),
            4 => Ok(VnStatus::Dropped),
            _ => Err(D::Error::custom(format!("Unknown type '{}' of VN's status.", status)))
        }
    }
}

impl Serialize for VnStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(*self as u8)
    }
}

#[derive(Deserialize, Serialize, Debug)]
///Vote list data representation. Returned by `get user`
pub struct VnList {
    ///Unique identifier of User.
    ///
    ///Provided when `basic` flag is specified.
    pub uid: Option<u64>,
    ///Unique identifier of VN.
    ///
    ///Provided when `basic` flag is specified.
    pub vn: Option<u64>,
    ///Vote value in range from 10 to 100.
    ///
    ///Provided when `basic` flag is specified.
    pub status: Option<VnStatus>,
    ///Unix timestamp of when this vote is added.
    ///
    ///Provided when `basic` flag is specified.
    pub added: Option<u64>,
    ///User's notes.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub notes: Option<String>
}
