extern crate foscam;
#[macro_use]
extern crate serde;
extern crate ron;

use foscam::zinc;
use ron::de::from_reader;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Config {
    url: String,
    auth: Auth,
}

#[derive(Debug, Deserialize)]
struct Auth {
    username: String,
    password: String,
}

fn main() {
    // get our config file
    let input_path = format!("{}/examples/config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            ::std::process::exit(1);
        }
    };

    // create our camera
    let cam = zinc::Camera {
        auth: zinc::Auth {
            username: &config.auth.username,
            password: &config.auth.password,
            url: &config.url,
        },
    };
}
