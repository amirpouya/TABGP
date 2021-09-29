use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Active{
    pub eid: usize,
    pub time: usize
}


impl  Active {
    pub fn new(eid: usize, time: usize) -> Self{
        Self{
            eid,
            time
        }

    }
    pub fn get_from_file(filename:&str) -> Vec<Active>
    {
        let mut active: Vec<Active> = vec![] ;
        let mut data_line = match File::open(&Path::new(&filename)) {
            Ok(file) => BufReader::new(file).lines(),
            Err(why) => panic!("EXCEPTION: couldn't open {}: {}",
                               Path::new(&filename).display(),
                               why.to_string(),
            )};
        // read the data
        for (_, line) in data_line.by_ref().enumerate() {
            let good_line = line.ok().expect("EXCEPTION: read error");
            if !good_line.starts_with('#') && good_line.len() > 0 {
                let mut elements = good_line[..].split(",");
                let eid:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let time:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let a = Active{ eid, time };
                active.push(a)
            }
        }
        return active;
    }





}