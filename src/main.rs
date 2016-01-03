#[macro_use] extern crate hyper;
extern crate rustc_serialize;
extern crate csv;
extern crate argparse;
extern crate config;

use std::io::Read;

use hyper::Client;
// use hyper::header::Connection;
// use hyper::header::Headers;
use rustc_serialize::json;
use std::path::Path;

use argparse::{ArgumentParser, Store};
use config::reader;

header! { (ClientKey, "Client-Key") => [String] }

#[derive(RustcDecodable, RustcEncodable)]
struct Event {
    id: usize,
    title: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct Events {
    events: Vec<Event>,
    count: usize,
    offset: usize,
    next: Option<String>,
    has_next: bool
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "<offset: {}, count: {}>", self.offset, self.count));
        for item in &self.events {
            try!(write!(f, "[#{}: {}]", item.id, item.title));
        }
        Ok(())
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Event \"{}\"]", self.title)
    }
}

fn load_csv(prefix: &String, key: &String, output: &Path) {
    let client = Client::new();
    let count = 10;
    let mut offset = 0;

    let mut wtr = csv::Writer::from_file(output).unwrap();
    loop {
        println!("page {}", offset/count + 1);
        let url = format!("{}/events?count={}&offset={}", prefix, count, offset);
        let mut res = client.get(&url)
                    .header(ClientKey(key.clone()))
                    .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let decoded: Events = json::decode(&body).unwrap();
        if !decoded.has_next {
            break;
        }

        for record in decoded.events.into_iter() {
            let result = wtr.encode(record);
            assert!(result.is_ok());
        }

        offset += count;
    }

    println!("Done");
}

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
