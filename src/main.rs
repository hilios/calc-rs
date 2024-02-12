#[macro_use] extern crate rocket;

mod http;
mod trie;
mod calc;

use http::requests::get as get_calc;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/calc", routes![get_calc])
}
