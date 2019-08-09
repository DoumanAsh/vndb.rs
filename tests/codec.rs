use vndb::protocol::message;
use vndb::codec::Codec;

use bytes::BufMut;
use tokio_codec::{Encoder, Decoder};

#[test]
fn decode_response_ok() {
    let message = b"ok\x04";
    let mut bytes = bytes::BytesMut::with_capacity(message.len());
    bytes.put(&message[..]);

    let mut codec = Codec::new();

    let result = codec.decode(&mut bytes);
    let result = result.unwrap().unwrap();

    match result {
        message::Response::Ok => assert!(true),
        _ => assert!(false, "Unexpected type of result")
    }

    assert_eq!(bytes.len(), 0);
}

#[test]
fn decode_response_ok_parts() {
    let message = b"ok\x04";
    let mut bytes_part = bytes::BytesMut::with_capacity(message.len());
    let mut bytes = bytes::BytesMut::with_capacity(message.len());
    bytes.put(&message[..]);

    let mut codec = Codec::new();

    let result = codec.decode(&mut bytes_part);
    assert!(result.unwrap().is_none());

    let result = codec.decode(&mut bytes);
    let result = result.unwrap().unwrap();

    match result {
        message::Response::Ok => assert!(true),
        _ => assert!(false, "Unexpected type of result")
    }

    assert_eq!(bytes.len(), 0);
}

#[test]
fn decode_response_error() {
    let message = b"error {\"id\":\"parse\", \"msg\":\"Invalid command or argument\"}\x04";
    let mut bytes = bytes::BytesMut::with_capacity(message.len());
    bytes.put(&message[..]);

    let mut codec = Codec::new();

    let result = codec.decode(&mut bytes);
    let result = result.unwrap().unwrap();

    match result {
        message::Response::Error(error) => {
            assert_eq!(error.id, "parse");
            assert_eq!(error.msg, "Invalid command or argument");
        },
        _ => assert!(false, "Unexpected type of result")
    }

    assert_eq!(bytes.len(), 0);

}

#[test]
fn decode_response_dbstats() {
    let message = b"dbstats {
        \"users\":49084,
        \"threads\":3998,
        \"tags\":1627,
        \"releases\":28071,
        \"producers\":3456,
        \"chars\":14046,
        \"posts\":52470,
        \"vn\":13051,
        \"traits\":1272}\x04";

    let mut bytes = bytes::BytesMut::with_capacity(message.len());
    bytes.put(&message[..]);

    let mut codec = Codec::new();

    let result = codec.decode(&mut bytes);
    let result = result.unwrap().unwrap();

    match result {
        message::Response::DBstats(stats) => {
            assert_eq!(stats.users, 49084);
            assert_eq!(stats.threads, 3998);
            assert_eq!(stats.tags, 1627);
            assert_eq!(stats.producers, 3456);
            assert_eq!(stats.chars, 14046);
            assert_eq!(stats.posts, 52470);
            assert_eq!(stats.vn, 13051);
            assert_eq!(stats.traits, 1272);
        },
        _ => assert!(false, "Unexpected type of result")
    }

    assert_eq!(bytes.len(), 0);
}

#[test]
fn decode_response_results() {
    let message = b"results {
        \"num\":1,
        \"more\":false,
        \"items\":[{
            \"id\": 17,
            \"title\": \"Ever17 -the out of infinity-\",
            \"original\": null,
            \"released\": \"2002-08-29\",
            \"languages\": [\"en\",\"ja\",\"ru\",\"zh\"],
            \"platforms\": [\"drc\",\"ps2\",\"psp\",\"win\"],
            \"anime\": []
        }]
    }\x04";

    let mut bytes = bytes::BytesMut::with_capacity(message.len());
    bytes.put(&message[..]);

    let mut codec = Codec::new();

    let result = codec.decode(&mut bytes);
    let result = result.unwrap().unwrap();

    match result {
        message::Response::Results(results) => {
            assert_eq!(results["num"], serde_json::json!(1));
            assert_eq!(results["more"], serde_json::json!(false));
            let results = results.vn().unwrap();

            assert_eq!(results.num, 1);
            assert_eq!(results.more, false);
            assert_eq!(results.len(), 1);
            assert_eq!(results.items.len(), 1);
            let item = &results.items[0];

            assert_eq!(item.id, 17);
            assert_eq!(item.title, Some("Ever17 -the out of infinity-".to_owned()));
            assert_eq!(item.original, None);
        },
        _ => assert!(false, "Unexpected type of result")
    }

    assert_eq!(bytes.len(), 0);
}

#[test]
fn encode_request_login_without_auth() {
    let login = message::request::Login {
        protocol: 2,
        client: "test",
        clientver: 0.666,
        creds: None,
    };
    let login = message::Request::Login(login);

    let mut bytes = bytes::BytesMut::new();

    let mut codec = Codec::new();
    let result = codec.encode(login, &mut bytes);
    assert!(result.is_ok());
    assert_eq!(&bytes[..], &b"login {\"protocol\":2,\"client\":\"test\",\"clientver\":0.666}\x04"[..])
}

#[test]
fn encode_request_login_with_auth() {
    let login = message::request::Login {
        protocol: 2,
        client: "test",
        clientver: 0.666,
        creds: Some(("username", "pass")),
    };
    let login = message::Request::Login(login);

    let mut bytes = bytes::BytesMut::new();

    let mut codec = Codec::new();
    let result = codec.encode(login, &mut bytes);
    assert!(result.is_ok());
    assert_eq!(&bytes[..], &b"login {\"protocol\":2,\"client\":\"test\",\"clientver\":0.666,\"username\":\"username\",\"password\":\"pass\"}\x04"[..]);
}

#[test]
fn encode_request_get_without_options() {
    let get = message::Request::Get(message::request::Get {
        kind: message::request::get::Type::vn(),
        flags: message::request::get::Flags::new().basic().anime(),
        filters: message::request::get::Filters::new().filter(vndb::filter!(title = "Lolka")).or(vndb::filter!(title = "lolka")),
        options: None
    });

    let mut bytes = bytes::BytesMut::new();

    let mut codec = Codec::new();
    let result = codec.encode(get, &mut bytes);
    assert!(result.is_ok());
    assert_eq!(&bytes[..], &b"get vn basic,anime (title = \"Lolka\" or title = \"lolka\")\x04"[..]);
}

#[test]
fn encode_request_get_wit_options() {
    let get = message::Request::Get(message::request::Get {
        kind: message::request::get::Type::release(),
        flags: message::request::get::Flags::new().basic().details(),
        filters: message::request::get::Filters::new().filter(vndb::filter!(title = "Lolka")).or(vndb::filter!(title = "lolka")),
        options: Some(message::request::get::Options {
            page: Some(2),
            results: None,
            sort: None,
            reverse: Some(true)
        })
    });

    let mut bytes = bytes::BytesMut::new();

    let mut codec = Codec::new();
    let result = codec.encode(get, &mut bytes);
    assert!(result.is_ok());
    assert_eq!(&bytes[..], &b"get release basic,details (title = \"Lolka\" or title = \"lolka\") {\"page\":2,\"reverse\":true}\x04"[..]);
}

#[test]
fn encode_request_dbstats() {
    let dbstats = message::Request::DBstats;

    let mut bytes = bytes::BytesMut::new();
    let mut codec = Codec::new();
    let result = codec.encode(dbstats, &mut bytes);
    assert!(result.is_ok());
    assert_eq!(&bytes[..], &b"dbstats\x04"[..])
}
