#[macro_use] extern crate rocket;

use requests::{get as get_calc, post as post_calc};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/calculator", routes![get_calc, post_calc])
}
