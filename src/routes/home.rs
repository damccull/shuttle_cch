use actix_web::get;


#[get("/")]
#[tracing::instrument]
async fn home() -> &'static str {
    "Let the Christmas Code Hunt begin!"
}
