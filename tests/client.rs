#![feature(async_await)]

use vndb::protocol::message;

use vndb::matsu;

#[tokio::test]
async fn client_should_send_message_over_tcp() {
    let get = message::request::Get {
        kind: message::request::get::Type::vn(),
        flags: message::request::get::Flags::new().basic().details(),
        filters: message::request::get::Filters::new().filter(vndb::filter!(title ~ "Kizuna")).or(vndb::filter!(title = "Kizuna")),
        options: Some(message::request::get::Options {
            page: Some(1),
            results: None,
            sort: None,
            reverse: Some(true)
        })
    };

    let mut client = matsu!(vndb::client::tokio::Client::connect_tcp()).expect("To connect");

    matsu!(client.send(message::request::Login::new(None).into())).expect("To send login");
    matsu!(client.send(message::Request::DBstats)).expect("To send DbStats");
    matsu!(client.send(get.into())).expect("To send Get");
    matsu!(client.flush()).expect("To flush");

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => (),
        response => panic!("Unexpected response={:?}", response),
    }

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}

#[tokio::test]
async fn client_should_send_message_over_tls() {
    let get = message::request::Get {
        kind: message::request::get::Type::vn(),
        flags: message::request::get::Flags::new().basic().details(),
        filters: message::request::get::Filters::new().filter(vndb::filter!(title ~ "Kizuna")).or(vndb::filter!(title = "Kizuna")),
        options: Some(message::request::get::Options {
            page: Some(1),
            results: None,
            sort: None,
            reverse: Some(true)
        })
    };

    let mut client = matsu!(vndb::client::tokio::Client::connect_tls()).expect("To connect");

    matsu!(client.send(message::request::Login::new(None).into())).expect("To send login");
    matsu!(client.send(message::Request::DBstats)).expect("To send DbStats");
    matsu!(client.send(get.into())).expect("To send Get");
    matsu!(client.flush()).expect("To flush");

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => (),
        response => panic!("Unexpected response={:?}", response),
    }

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match matsu!(client.receive()).expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}
