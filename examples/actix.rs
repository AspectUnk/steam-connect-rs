use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use env_logger::Env;
use steam_connect::{Redirect, Verify};

async fn login() -> Result<HttpResponse> {
    Ok(Redirect::new("http://127.0.0.1:8080/auth/callback")
        .unwrap()
        .redirect())
}

async fn callback(req: HttpRequest) -> Result<HttpResponse> {
    Ok(match Verify::verify_request(req.query_string()).await {
        Ok(v) => HttpResponse::Ok().body(format!(
            "Hello {}! Your SteamID: {}",
            v.get_summaries("0D933E42846EE109646AD279C464376A")
                .await
                .unwrap()
                .personaname,
            v.claim_id(),
        )),
        Err(e) => HttpResponse::Unauthorized().body(format!("Err: {:?}", e)),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/auth")
                .route("/login", web::get().to(login))
                .route("/callback", web::get().to(callback)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
