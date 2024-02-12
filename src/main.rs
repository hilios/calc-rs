#[macro_use] extern crate rocket;

use rocket::tokio::time::{Duration, sleep};

mod trie;
mod calc;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![delay])
}
