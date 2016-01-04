#[macro_use] extern crate hyper;
extern crate rustc_serialize;
extern crate csv;

extern crate argparse;
extern crate config;

use std::path::Path;

use argparse::{ArgumentParser, Store};
use config::reader;

mod events_loader;

use events_loader::load_csv;


fn main() {
    let key;
    let api_prefix;

    { // limit config_file lifetime
        let mut config_file = "config.conf".to_string();
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("EventBaboon");
            ap.refer(&mut config_file)
                .add_option(&["--config-file"], Store,
                "Config file name");
            ap.parse_args_or_exit();
        }
        println!("Using {} config file", config_file);

        {
            let conf = reader::from_file(Path::new(&config_file));
            assert!(conf.is_ok());
            let conf = conf.unwrap();
            key = conf.lookup_str("events-api.client-key").unwrap().to_owned();
            api_prefix = conf.lookup_str("events-api.api-prefix").unwrap().to_owned();
        }
    }
    let path = Path::new("./output.csv");
    load_csv(&api_prefix, &key, path);
}
