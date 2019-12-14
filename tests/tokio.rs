use vndb::protocol::message;

#[cfg(feature = "tokio-on")]
#[tokio::test]
async fn tokio_client_should_send_message_over_tcp() {
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

    let mut client = vndb::client::tokio::Client::connect().await.expect("To connect");
    client.reconnect().await.expect("Reconnect");

    client.send(&message::request::Login::new(None).into()).await.expect("To send login");
    client.send(&message::Request::DBstats).await.expect("To send DbStats");
    client.send(&get.into()).await.expect("To send Get");
    client.flush().await.expect("To flush");


    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => println!("Ok"),
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}

#[cfg(all(feature = "tokio-on", feature = "rustls-on"))]
#[tokio::test]
async fn tokio_client_should_send_message_over_tls() {
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

    let mut client = vndb::client::tokio::Client::connect_tls().await.expect("To connect");
    client.reconnect_tls().await.expect("Reconnect");

    client.send(&message::request::Login::new(None).into()).await.expect("To send login");
    client.send(&message::Request::DBstats).await.expect("To send DbStats");
    client.send(&get.into()).await.expect("To send Get");
    client.flush().await.expect("To flush");


    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::Ok => println!("Ok"),
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::DBstats(response) => {
            println!("DBstats={:?}", response);
        },
        response => panic!("Unexpected response={:?}", response),
    }

    match client.receive().await.expect("To receive message").expect("To not fail receiving") {
        message::Response::Results(response) => {
            let results = response.vn().expect("Parse into VN Results");
            println!("Get Results={:?}", results);
        }
        response => panic!("Unexpected response={:?}", response),
    }
}
