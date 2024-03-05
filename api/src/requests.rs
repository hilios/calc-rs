use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use shared::calc::Calc;

#[derive(Serialize, Deserialize)]
pub struct CalcRequest<'r> {
    input: &'r str,
    infix: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CalcResponse {
    output: String,
}

#[get("/", data = "<request>")]
pub fn get(request: Json<CalcRequest<'_>>) -> Json<Calc> {
    let calc = if request.infix.unwrap_or(false) {
        Calc::infix(request.input)
    } else {
        Calc::postfix(request.input)
    }
    .unwrap();
    Json(calc)
}

#[post("/", data = "<request>")]
pub fn post(request: Json<CalcRequest<'_>>) -> Json<CalcResponse> {
    let calc = if request.infix.unwrap_or(false) {
        Calc::infix(request.input)
    } else {
        Calc::postfix(request.input)
    }
    .unwrap();
    Json(CalcResponse {
        output: calc.to_string(),
    })
}
