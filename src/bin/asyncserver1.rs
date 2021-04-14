use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::Client;

#[actix_web::main]
async fn main() {
    let server = server_run();
    let mut requests = FuturesUnordered::new();
    for i in 1..=1000 {
        let url = format!("http://localhost:4444/number/{}", i);
        requests.push(Client::default().get(&url).send());
    }
    while let Some(resp) = requests.next().await {
        match resp {
            Err(e) => eprintln!("{:?}", e),
            Ok(resp) => println!("<== {:?}", resp.text().await.unwrap()),
        }
    }
    server.stop(/*graceful*/ true).await;
}

fn server_run() -> Server {
    HttpServer::new(|| {
        App::new().route(
            "/number/{n}",
            web::get().to(|req: HttpRequest| {
                let n = req.match_info().get("n").unwrap();
                let body = format!("Hello number {}!", n);
                HttpResponse::Ok().body(body)
            }),
        )
    })
    .bind(("127.0.0.1", 4444))
    .unwrap()
    .run()
}
