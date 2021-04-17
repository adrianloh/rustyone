use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::{FuturesUnordered, StreamExt};
use rand::Rng;
use reqwest::Client;
use std::iter::repeat_with;
use std::ops::AddAssign;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

#[actix_web::main]
async fn main() {
    let server = server_run(); // `server` is `await`-able but we don't want to

    let mut requests = FuturesUnordered::new();

    for _ in 0..100 {
        // GET `total` random bytes from server
        let total = rand::thread_rng().gen_range(10000..100000);
        let url = format!("http://localhost:5555/bytes/{}", total);
        let fut = Client::default().get(url).send();
        requests.push(fut);
    }

    let mut bytes_received = 0;

    // `requests.next()` returns as futures complete. They are unordered.
    // When all futures are done, it returns `None`, which causes
    // this `while` loop to break
    while let Some(resp) = requests.next().await {
        // bytes_received += response.headers["content-length"]
        bytes_received.add_assign(match resp {
            Err(_e) => 0,
            Ok(resp) => match resp.headers().get("content-length") {
                Some(v) => u64::from_str(v.to_str().unwrap()).unwrap_or(0),
                None => 0,
            },
        });
    }
    println!("{:>11}: {} bytes", "total", bytes_received);

    // Shut the server down _gracefully_
    server.stop(/*graceful*/ true).await;
}

// Start a server with one route
fn server_run() -> Server {
    HttpServer::new(|| {
        App::new()
            // GET {total} random bytes
            .route("/bytes/{total}", web::get().to(handler_rand_bytes))
    })
    .bind(("127.0.0.1", 5555))
    .unwrap()
    .run()
}

// Returns an HttpResponse of random bytes
async fn handler_rand_bytes(req: HttpRequest) -> HttpResponse {
    // grab {total} from the url path "/bytes/{total}"
    let total_str = req.match_info().get("total").unwrap_or("0");
    let total = u64::from_str(total_str).unwrap();

    // build a vector of random bytes of `total` length
    let random_byte = repeat_with(rand::random::<u8>);
    let bytes: Vec<_> = random_byte.take(total as usize).collect();

    println!("{:>11}: {} bytes", "requested", total_str);

    // Simulate response latency
    let latency = rand::thread_rng().gen_range(1000..=2000);
    sleep(Duration::from_millis(latency)).await;

    println!("{:>11}: {} bytes", "sending", total_str);

    HttpResponse::Ok()
        .append_header(("content-type", "application/octet-stream"))
        .body(bytes)
}
