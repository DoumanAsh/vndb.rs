
use vndb::protocol::message;

#[test]
fn simple_client_should_send_message_over_tcp() {
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

    let mut client = vndb::client::simple::Client::connect().expect("To connect");
    client.reconnect().expect("Reconnect");

    client.send(&message::request::Login::new(None).into()).expect("To send login");
    client.send(&message::Request::DBstats).expect("To send DbStats");
    client.send(&get.into()).expect("To send Get");
    client.flush().expect("To flush");


    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => println!("Ok"),
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}

#[cfg(feature = "rustls-on")]
#[test]
fn simple_tls_client_should_send_message_over_tcp() {
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

    let mut client = vndb::client::simple::Client::connect_tls().expect("To connect");
    client.reconnect_tls().expect("Reconnect");

    client.send(&message::request::Login::new(None).into()).expect("To send login");
    client.send(&message::Request::DBstats).expect("To send DbStats");
    client.send(&get.into()).expect("To send Get");
    client.flush().expect("To flush");


    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => println!("Ok"),
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}
