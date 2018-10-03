use rocket_contrib::json::Json;
use std::any::Any

#[derive(Serialize, Deserialize)]
pub struct JsonResult<D> {
    data: D
}

pub fn json_response<A>(data: A) -> Json<JsonResult<A>> {
    Json(JsonResult { data })
}

trait Controller {
    fn handlers() -> Vec<(Any -> Any)>;
}