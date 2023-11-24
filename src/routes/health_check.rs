use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
#[tracing::instrument]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy. Enough said.")
}
