use std;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Event {
    pub id: usize,
    pub title: String,
    pub agenda: String,
    pub social: String,
    pub image_url: Option<String>,
    pub level: String,
    pub place: Option<String>,
    pub when_start: String,
    pub when_end: Option<String>,
    pub only_date: bool,
    pub registration_url: String,
    pub special: bool,
    pub provider: usize,
    pub metainfo: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Events {
    pub events: Vec<Event>,
    pub count: usize,
    pub offset: usize,
    pub next: Option<String>,
    pub has_next: bool
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
