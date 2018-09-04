use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct JsonResult<D> {
    data: D
}

pub fn json_response<A>(data: A) -> Json<JsonResult<A>> {
    Json(JsonResult { data })
}