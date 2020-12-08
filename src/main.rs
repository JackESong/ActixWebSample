use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "index!"
}

async fn hello(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/index").to(index))
            .service(web::resource("/hello").to(hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
