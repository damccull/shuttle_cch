use actix_web::{get, HttpResponse, Responder};

#[get("/")]
#[tracing::instrument]
async fn home() -> &'static str {
    "Let the Christmas Code Hunt begin!"
}

#[get("/-1/error")]
#[tracing::instrument]
pub async fn bonus_return_error() -> impl Responder {
    HttpResponse::InternalServerError()
        .body("500 error. Usually sad, but this time expected. Bonus task passed.")
}
