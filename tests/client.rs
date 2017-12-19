#[macro_use]
extern crate vndb;
extern crate tokio_core;
extern crate futures;

use futures::Future;

use vndb::protocol::message;

fn init_client() -> (tokio_core::reactor::Core, vndb::client::tokio::Client) {
    let mut tokio_core = tokio_core::reactor::Core::new().expect("Should create tokio core");
    let client = vndb::client::tokio::Client::new(&tokio_core.handle()).expect("Should initialize client");

    let client = tokio_core.run(client).expect("Pending connect should be successful");
    (tokio_core, client)
}


#[test]
fn send_messages() {
    let (mut tokio_core, client) = init_client();

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

    let send = client.send(message::request::Login::new(None, None)).map_err(|_| "Successful Login send")
               .and_then(|sender| sender.send(message::Request::DBstats).map_err(|_| "Successful DBstats send"))
               .and_then(move |sender| sender.send(get).map_err(|_| "Successful Get vn send"));

    tokio_core.run(send).expect("Should send multiple requests!");

    let receive = client.receive()
                        .and_then(|(rsp, receiver)| match rsp {
                            Some(message::Response::Ok) => receiver.receive(),
                            response => panic!(format!("Unexpected response={:?}", response))
                        }).and_then(|(rsp, receiver)| match rsp {
                            Some(message::Response::DBstats(response)) => {
                                println!("DBstats={:?}", response);
                                receiver.receive()
                            },
                            response => panic!(format!("Unexpected response={:?}", response))
                        }).map(|(rsp, _)| match rsp {
                            Some(message::Response::Results(response)) => {
                                let results = response.vn().expect("Parse into VN Results");
                                println!("Get Results={:?}", results);
                            },
                            response => panic!(format!("Unexpected response={:?}", response))
                        });

    tokio_core.run(receive).expect("Should receive multiple requests!");
}
