use actix_web::{get, web::Path, HttpRequest, HttpResponse, Responder};

#[tracing::instrument]
// #[get(r#"/1/{tail:(?:\/\d+)+}"#)]
#[get(r#"/1/{tail:.*}"#)]
pub async fn xor_power3(req: HttpRequest, path: Path<String>) -> impl Responder {
    let split = path.split("/");
    let nums = split
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();
    let result: i32;
    if let Ok(nums) = nums {
        let initial = nums[0];
        let r = nums.iter().skip(1).fold(initial, |a: i32, n| a ^ n);
        result = i32::pow(r, 3);
    } else {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().body(result.to_string())
}
