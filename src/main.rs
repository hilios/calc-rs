#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
mod trie;
mod calc;

use trie::{Trie};

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
    let mut trie = Trie::new();
    trie.insert("test");

    trie.starts_with("te");

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![delay])
}
