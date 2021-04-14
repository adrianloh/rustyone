use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::{FuturesUnordered, StreamExt};
use rand::Rng;
use reqwest::Client;
use std::iter::repeat_with;
use std::ops::AddAssign;
use std::str::FromStr;

#[actix_web::main]
async fn main() {
    let server = server_run();
    let mut requests: FuturesUnordered<_> = (0..1000)
        .into_iter()
        .map(|_| Client::default().get("http://localhost:5555/bytes").send())
        .collect();
    println!("total requests: {}", requests.len());
    let mut total_bytes = 0;
    while let Some(resp) = requests.next().await {
        total_bytes.add_assign(match resp {
            Err(_e) => 0,
            Ok(resp) => match resp.headers().get("content-length") {
                Some(v) => u64::from_str(v.to_str().unwrap()).unwrap_or(0),
                None => 0,
            },
        });
    }
    println!("total bytes: {}", total_bytes);
    server.stop(/*graceful*/ true).await;
}

fn server_run() -> Server {
    HttpServer::new(|| {
        let random_byte = repeat_with(rand::random::<u8>);
        App::new().route(
            "/bytes",
            web::get().to(move |_req: HttpRequest| {
                let total = rand::thread_rng().gen_range(10000..100000);
                let bytes: Vec<_> = random_byte.take(total).collect();
                HttpResponse::Ok()
                    .append_header(("content-type", "application/octet-stream"))
                    .body(bytes)
            }),
        )
    })
    .bind(("127.0.0.1", 5555))
    .unwrap()
    .run()
}
