#[macro_use]
extern crate vndb;
extern crate tokio;
extern crate futures;

use futures::{Future, Stream};

use vndb::protocol::message;

#[test]
fn send_messages() {
    let get = message::request::Get {
        kind: message::request::get::Type::vn(),
        flags: message::request::get::Flags::new().basic().details(),
        filters: message::request::get::Filters::new().filter(filter!(title ~ "Kizuna")).or(filter!(title = "Kizuna")),
        options: Some(message::request::get::Options {
            page: Some(1),
            results: None,
            sort: None,
            reverse: Some(true)
        })
    };

    let client = vndb::client::tokio::Client::new().expect("Create VNDB Client");
    let test = client.map_err(|error| panic!("Failed to connect to VNDB: {}", error))
                     .map(move |client| {
                         println!("Start sending requests");
                         client.send(message::request::Login::new(None, None)).expect("Unable to send login");
                         client.send(message::Request::DBstats).expect("Unable to send DBstats");
                         client.send(get).expect("Unable to send Get VN");
                         client
                     }).and_then(|stream| {
                         stream.into_future().map_err(|_| panic!("Error getting login response"))
                     }).and_then(|(login, stream)| match login {
                         Some(message::Response::Ok) => stream.into_future().map_err(|_| panic!("Error getting DBstats response")),
                         response => panic!("Unexpected response={:?}", response)
                     }).and_then(|(db_stats, stream)| match db_stats {
                         Some(message::Response::DBstats(response)) => {
                             println!("DBstats={:?}", response);
                             stream.into_future()
                                   .map_err(|_| panic!("Error getting Get VN response"))
                         },
                         response => panic!("Unexpected response={:?}", response)
                     }).and_then(|(get, _stream)| match get {
                         Some(message::Response::Results(response)) => {
                             let results = response.vn().expect("Parse into VN Results");
                             println!("Get Results={:?}", results);
                             Ok(())
                         },
                         response => panic!(format!("Unexpected response={:?}", response))
                     });

    tokio::run(test);
}
