use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::calc::expr::Expr;
use crate::calc::Calc;

#[derive(Serialize, Deserialize)]
pub struct CalcRequest<'r> {
    input: &'r str,
    infix: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CalcResponse {
    output: String,
    memory: Vec<Expr>,
}

#[get("/calculator", data = "<request>")]
pub fn get(request: Json<CalcRequest<'_>>) -> Json<CalcResponse> {
    let calc = if request.infix.unwrap_or(false) {
        Calc::infix(request.input)
    } else {
        Calc::postfix(request.input)
    }.unwrap();
    Json(CalcResponse {
        output: calc.to_string(),
        memory: calc.memory.clone(),
    })
}