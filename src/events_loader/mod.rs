pub mod types;

use std::io::Read;
use hyper::Client;
use rustc_serialize::json;
use csv;

use std::path::Path;
use self::types::Events;


header! { (ClientKey, "Client-Key") => [String] }


pub fn load_csv(prefix: &String, key: &String, output: &Path) {
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
