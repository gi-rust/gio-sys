extern crate pkg_config;

use pkg_config::Config;

fn main() {
    Config::new().atleast_version("2.47").find("gio-2.0").unwrap();
}
