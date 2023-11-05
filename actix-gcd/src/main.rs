use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(post_gcd)
        .route("/", web::get().to(get_index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}


async fn get_index() -> impl Responder {
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

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type(ContentType::html())
            .body("Computing the GCD with zero is boring");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} \
                        is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(response)
}

/// 最大公约数函数
fn gcd(mut n: u64, mut m:u64) -> u64{
    assert!(n != 0 && m!= 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// 参数定义
#[derive(Deserialize)]
struct GcdParameters{
    n:u64,
    m:u64,
}