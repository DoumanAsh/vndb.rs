use vndb::{filter};
use vndb::protocol::message;

use serde_json::json;

#[test]
fn get_type_short() {
    let vn_type = message::request::get::Type::vn();

    assert_eq!(vn_type.short(), "v");
}

#[test]
fn format_request_login_without_auth() {
    let login = message::request::Login {
        protocol: 2,
        client: "test",
        clientver: 0.666,
        creds: None,
    };
    let login = message::Request::Login(login);

    let result = format!("{}", login);
    assert_eq!(result, "login {\"protocol\":2,\"client\":\"test\",\"clientver\":0.666}\x04")
}

#[test]
fn format_request_login_with_auth() {
    let login = message::request::Login {
        protocol: 2,
        client: "test",
        clientver: 0.666,
        creds: Some(("username", "pass")),
    };
    let login = message::Request::Login(login);

    let result = format!("{}", login);
    assert_eq!(result, "login {\"protocol\":2,\"client\":\"test\",\"clientver\":0.666,\"username\":\"username\",\"password\":\"pass\"}\x04")
}

#[test]
fn format_request_get_without_options() {
    let get = message::Request::Get(message::request::Get {
        kind: message::request::get::Type::vn(),
        flags: message::request::get::Flags::new().basic().anime(),
        filters: message::request::get::Filters::new().filter(filter!(title = "Lolka")).or(filter!(title = "lolka")),
        options: None
    });

    let result = format!("{}", get);
    assert_eq!(result, "get vn basic,anime (title = \"Lolka\" or title = \"lolka\")\x04");
}

#[test]
fn format_request_get_wit_options() {
    let get = message::Request::Get(message::request::Get {
        kind: message::request::get::Type::release(),
        flags: message::request::get::Flags::new().basic().details(),
        filters: message::request::get::Filters::new().filter(filter!(title = "Lolka")).or(filter!(title = "lolka")),
        options: Some(message::request::get::Options {
            page: Some(2),
            results: None,
            sort: None,
            reverse: Some(true)
        })
    });

    let result = format!("{}", get);
    assert_eq!(result, "get release basic,details (title = \"Lolka\" or title = \"lolka\") {\"page\":2,\"reverse\":true}\x04");

}

#[test]
fn format_request_dbstats() {
    let dbstats = message::Request::DBstats;

    let result = format!("{}", dbstats);
    assert_eq!(result, "dbstats\x04")
}

#[test]
fn parse_response_ok() {
    let message = "ok";
    let result = message::Response::from_str(message).expect("To parse");
    match result {
        message::Response::Ok => assert!(true),
        _ => assert!(false, "Unexpected type of result")
    }
}

#[test]
fn parse_response_error() {
    let message = "error {\"id\":\"parse\", \"msg\":\"Invalid command or argument\"}";

    let result = message::Response::from_str(message).expect("To parse");
    match result {
        message::Response::Error(error) => {
            assert_eq!(error.id, "parse");
            assert_eq!(error.msg, "Invalid command or argument");
        },
        _ => assert!(false, "Unexpected type of result")
    }
}

#[test]
fn parse_response_dbstats() {
    let message = "dbstats {
        \"tags\":1627,
        \"releases\":28071,
        \"producers\":3456,
        \"chars\":14046,
        \"vn\":13051,
        \"traits\":1272}";

    let result = message::Response::from_str(message).expect("To parse");

    match result {
        message::Response::DBstats(stats) => {
            assert_eq!(stats.tags, 1627);
            assert_eq!(stats.producers, 3456);
            assert_eq!(stats.chars, 14046);
            assert_eq!(stats.vn, 13051);
            assert_eq!(stats.traits, 1272);
        },
        _ => assert!(false, "Unexpected type of result")
    }
}

#[test]
fn parse_response_results() {
    let message = "results {
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
    }";

    let result = message::Response::from_str(message).expect("To parse");
    match result {
        message::Response::Results(results) => {
            assert_eq!(results["num"], json!(1));
            assert_eq!(results["more"], json!(false));
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
}
