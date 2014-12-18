use std::io::{BufferedReader,File};
use std::vec::Vec;

#[deriving(Clone)]
pub struct GECOS {
    pub name : String,
    pub location : String,
    pub telephone : String,
    pub other : Vec<String>,
}

#[deriving(Clone)]
pub struct Entry {
    pub username : String,
    pub gecos : GECOS,
    pub home : String,
    pub shell : String,
}

fn parse_gecos(s:&str) -> GECOS {
    let mut split = s.split(',').map(|x| String::from_str(x));
    let name = split.next().unwrap_or(String::from_str(""));
    let loc = split.next().unwrap_or(String::from_str(""));
    let tel = split.next().unwrap_or(String::from_str(""));
    let mut other = Vec::new();
    for o in split {
        other.push(o);
    }
    GECOS{ name:name,location:loc,telephone:tel,other:other }
}

fn parse_line(s:&str) -> Entry {
    let mut splits = s.split(':').map(|x| String::from_str(x));
    let un = splits.next().unwrap();
    // skip next three
    splits.next(); splits.next(); splits.next();
    let gecos = parse_gecos(splits.next().unwrap().as_slice());
    let home = splits.next().unwrap();
    let shell = splits.next().unwrap();
    Entry{ username:un, gecos:gecos, home:home, shell:shell }
}

fn get_all_entries() -> Vec<Entry> {
    let mut v = Vec::new();
    let pw_path = Path::new("/etc/passwd");
    let mut f = BufferedReader::new(File::open(&pw_path));
    for line in f.lines() {
        v.push(parse_line(line.unwrap().as_slice()));
    }
    return v;
}

pub fn get_entry(username : String) -> Option<Entry> {
    let passwds = get_all_entries();
    for entry in passwds.iter() {
        if entry.username == username { return Some(entry.clone()); }
    }
    None
//    Some(Entry{ username: username, gecos : g, home: home, shell:shell })
}
