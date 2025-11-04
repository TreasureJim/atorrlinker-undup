mod plugins;

use std::path::PathBuf;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

struct Config {
    plugin_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            plugin_path: std::env::var("ATORRLINKER_PLUGS")
                .expect("Could not find ATORRLINKER_PLUGS environment variable.")
                .into(),
        }
    }
}

#[rocket::main]
async fn main() {
    if cfg!(debug_assertions) {
        // Ignore errors if there is no env file
        let _ = dotenv::from_filename(".dev.env");
    }

    let config = Config::new();

    pyo3::Python::initialize();

    // Load plugin modules
    // Load plugin functions and arguments

    let _ = rocket().launch().await;
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, hello])
}

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
