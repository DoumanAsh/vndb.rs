#[macro_use]
extern crate vndb;
extern crate tokio_core;

use vndb::protocol::message;

fn init_client() -> (tokio_core::reactor::Core, vndb::client::Client) {
    let mut tokio_core = tokio_core::reactor::Core::new().expect("Should create tokio core");
    let client = vndb::client::Client::new(&tokio_core.handle()).expect("Should initialize client");

    let client = tokio_core.run(client).expect("Pending connect should be successful");
    (tokio_core, client)
}


#[test]
fn send_messages() {
    let (mut tokio_core, mut client) = init_client();

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

    client.send(message::request::Login::new(None, None)).expect("Successful Login send")
          .send(message::Request::DBstats).expect("Successful DBstats send")
          .send(get).expect("Successful Get vn send");

    let response = tokio_core.run(client.receive()).expect("Successful receive").expect("Non empty receive");
    match response {
        message::Response::Ok => (),
        _ => assert!(false, format!("Unexpected response={:?}", response))
    }

    let response = tokio_core.run(client.receive()).expect("Successful receive").expect("Non empty receive");
    match response {
        message::Response::DBstats(response) => println!("{:?}", response),
        _ => assert!(false, format!("Unexpected response={:?}", response))
    }

    let response = tokio_core.run(client.receive()).expect("Successful receive").expect("Non empty receive");
    match response {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("{:?}", results);
        },
        _ => assert!(false, format!("Unexpected response={:?}", response))
    }
}
