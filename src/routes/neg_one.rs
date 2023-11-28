use actix_web::{HttpResponse, get, Responder};


#[get("/-1/error")]
#[tracing::instrument]
pub async fn neg_one_bonus_return_error() -> impl Responder {
    HttpResponse::InternalServerError().body("500 error. Usually sad, but this time expected. Bonus task passed.")
}
