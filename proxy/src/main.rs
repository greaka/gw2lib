use std::{sync::Mutex, time::Duration};

use actix_web::{http::header::HeaderName, web, HttpRequest, HttpResponse, Responder};
use gw2api_http::rate_limit::{BucketRateLimiter, RateLimiter};
use reqwest::{header::HeaderValue, Method};

#[actix_web::main]
async fn main() {
    let rate = BucketRateLimiter::default();
    let rate = web::Data::new(Mutex::new(rate));
    let app = move || {
        actix_web::App::new()
            .app_data(rate.clone())
            .default_service(actix_web::web::get().to(index))
    };
    actix_web::HttpServer::new(app)
        .bind("0.0.0.0:52321")
        .unwrap()
        .run()
        .await
        .unwrap();
}

async fn index(req: HttpRequest) -> impl Responder {
    let uri = ("https://api.guildwars2.com".to_string() + req.uri().to_string().as_str())
        .parse()
        .unwrap();
    let mut spoof = reqwest::Request::new(Method::GET, uri);
    req.headers()
        .iter()
        .filter(|(h, _)| h != &HeaderName::from_static("host"))
        .for_each(|(h, v)| {
            spoof.headers_mut().append(h, v.clone());
        });
    spoof
        .headers_mut()
        .append("host", HeaderValue::from_static("api.guildwars2.com"));
    let rate = req
        .app_data::<web::Data<Mutex<BucketRateLimiter>>>()
        .unwrap();
    let dur = { rate.lock().unwrap().take(1) };
    tokio::time::sleep(Duration::from_secs(dur)).await;
    println!("{} - {:?}", spoof.url(), spoof.headers());
    let res = reqwest::Client::default().execute(spoof).await;
    match res {
        Ok(r) => {
            let mut res = HttpResponse::new(r.status());
            r.headers().iter().for_each(|(h, v)| {
                res.headers_mut().append(h.clone(), v.clone());
            });
            let body = r.bytes().await.map_err(|e| {
                let e: Box<dyn std::error::Error> = Box::new(e);
                e
            })?;
            Ok(res.set_body(body))
        }
        Err(e) => Err({
            let e: Box<dyn std::error::Error> = Box::new(e);
            e
        }),
    }
}
