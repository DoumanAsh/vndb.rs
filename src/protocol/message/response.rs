use ::serde::{Deserialize};
use ::serde_json;

use ::fmt;

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

///Strongly typed variants for Results.
pub mod results {
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
    pub fn vn(&self) -> serde_json::Result<ResultsVN> {
        self.to()
    }

    #[inline]
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
