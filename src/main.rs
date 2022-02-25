use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct User {
    name: String,
    second_name: String,
    age: u8
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(data: web::Json<User>) -> impl Responder {
    let app_name = &data.name;
    
    HttpResponse::Ok().body(app_name)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
