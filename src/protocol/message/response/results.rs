//! VNDB entities types.
use serde::{Serialize, Deserialize};
use serde::de::Error;

#[derive(Deserialize, Serialize, Debug)]
///Links for VN.
///
///All fields may be omitted by VNDB.
pub struct VnLinks {
    ///Wikipedia identifier for the VN
    pub wikidata: Option<String>,
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
    ///Release's flags
    pub flagging: Option<VnImageFlags>,
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
///Flags of the main VN image
pub struct VnImageFlags {
    ///number of flagging votes.
    #[serde(rename = "votecount")]
    pub vote_count: usize,
    ///Sexual score between 0 (safe) and 2 (explicit).
    pub sexual_avg: Option<f32>,
    ///Violence score between 0 (tame) and 2 (brutal).
    pub violence_avg: Option<f32>,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Languages in which VN is available.
    ///
    ///Provided when `basic` flag is specified.
    ///Can be empty array.
    pub languages: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Languages of the first release.
    ///
    ///Optionally provided when `basic` flag is specified.
    ///Can be empty array.
    pub orig_lang: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Platforms on which VN is available.
    ///
    ///Optionally provided when `basic` flag is specified.
    ///Can be empty array.
    pub platforms: Vec<String>,

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
    ///Flags of the `image`
    pub image_flagging: Option<VnImageFlags>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Anime
    ///List of anime related to the VN.
    ///
    ///Provided when `anime` flag is specified.
    pub anime: Vec<VnAnime>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Relations
    ///List of related VNs.
    ///
    ///Provided when `relations` flag is specified.
    pub relations: Vec<VnRelation>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Tags
    ///List of VN's tags
    ///
    ///Provided when `tags` flag is specified.
    pub tags: Vec<VnTag>,

    #[serde(default)]
    //Stats
    ///Popularity from 0 to 100.
    ///
    ///Provided when `stats` flag is specified.
    pub popularity: f32,
    ///VN's rating from 1 to 10.
    ///
    ///Provided when `stats` flag is specified.
    pub rating: Option<f32>,
    ///Number of votes
    ///
    ///Provided when `stats` flag is specified.
    pub votecount: Option<u64>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Screens
    ///List of screenshots
    ///
    ///Provided when `screens` flag is specified.
    pub screens: Vec<VnScreen>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Staff
    ///List of Staff members.
    ///
    ///Provided when `staff` flag is specified.
    pub staff: Vec<VnStaff>,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Languages in which release is available.
    ///
    ///Provided when `basic` flag is specified.
    ///Can be empty array.
    pub languages: Vec<String>,

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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Platforms on which release is available.
    ///
    ///Optionally provided when `details` flag is specified.
    ///Can be empty array.
    pub platforms: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Release's media.
    ///
    ///Optionally provided when `details` flag is specified.
    pub media: Vec<ReleaseMedia>,
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Related VNs.
    ///
    ///Optionally provided when `vn` flag is specified.
    pub vn: Vec<ReleaseVN>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///Related producers.
    ///
    ///Optionally provided when `producer` flag is specified.
    pub producers: Vec<ReleaseProducer>,
}

#[derive(Deserialize, Serialize, Debug)]
///External links related for [Producer](struct.Prodcer.html)
pub struct ProducerLinks {
    ///Official homepage.
    pub homepage: Option<String>,
    ///Wikidata identifier.
    pub wikidata: Option<String>
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

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Relations
    ///List of related producers.
    ///
    ///Provided when `relations` flag is specified.
    pub relations: Vec<ProducerRelation>,
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
            _ => Err(D::Error::custom(format_args!("Unknown type '{}' of character gender.", gender)))
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

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    //Traits
    ///List, possibly empty, of traits specified as tuple `(id, spoiler level)`.
    ///
    ///Provided when `traits` flag is specified.
    pub traits: Vec<(u64, u8)>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///List, possibly empty, of related VNs specified as tuple `(vn id, release id, spoiler level, role)`.
    ///
    ///Provided when `vns` flag is specified.
    pub vns: Vec<(u64, u64, u8, String)>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ///List, possibly empty, of related VNs specified as tuple `(vn id, release id, spoiler level, role)`.
    ///
    ///Provided when `voiced` flag is specified.
    pub voiced: Vec<CharacterSeiyuu>,
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
            _ => Err(D::Error::custom(format_args!("Unknown type '{}' of VN's status.", status)))
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
    ///Status of VN.
    pub status: Option<VnStatus>,
    #[serde(default)]
    ///Unix timestamp of when this vote is added.
    ///
    ///Provided when `basic` flag is specified.
    pub added: u64,
    ///User's notes.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub notes: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
///Vote list data representation. Returned by `get user`
pub struct UList {
    ///Unique identifier of User.
    ///
    ///Provided when `basic` flag is specified.
    pub uid: Option<u64>,
    ///Unique identifier of VN.
    ///
    ///Provided when `basic` flag is specified.
    pub vn: Option<u64>,
    #[serde(default)]
    ///Unix timestamp of when this item has been added.
    ///
    ///Provided when `basic` flag is specified.
    pub added: u64,
    #[serde(rename = "lastmod", default)]
    ///Unix timestamp of when this item has been last modified.
    ///
    ///Provided when `basic` flag is specified.
    pub last_mod: u64,
    #[serde(default)]
    ///Unix timestamp when the vote has been cast.
    ///
    ///Provided when `basic` flag is specified.
    pub voted: u64,
    ///Vote value in range from 10 to 100.
    ///
    ///Provided when `basic` flag is specified.
    pub vote: Option<u8>,
    ///User's notes.
    ///
    ///Optionally provided when `basic` flag is specified.
    pub notes: Option<String>
}
