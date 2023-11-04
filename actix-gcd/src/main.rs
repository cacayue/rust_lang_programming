use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(_get_index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}


async fn _get_index() -> impl Responder {
    let html = r#"
    <html>
        <head>
            <title>GCD Calculator</title>
        </head>
        <body>
            <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">Compute GCD</button>
            </form>
        </body>
    </html>
    "#;

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}