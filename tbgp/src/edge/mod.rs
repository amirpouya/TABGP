use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Edge{
    pub eid: usize,
    pub src: usize,
    pub dst: usize,
    pub label: usize,
    pub first: usize
}


impl Edge {

    pub fn default() -> Edge
    {
        return Edge::new(0,0,0,0,0)
    }
    pub fn new(eid: usize, src:usize, dst:usize,label:usize, first:usize) -> Self {
        Self {
            eid,
            src,
            dst,
            label,
            first
        }
    }


    pub fn get_from_file(filename:&str) -> Vec<Edge>
    {
        let mut edges: Vec<Edge> = vec![] ;
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
                let src:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let dst:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let label: usize = 0;
                let first:usize = elements.next().unwrap_or("0").parse().ok().expect("malformed src");
                let e = Edge{ eid,src,dst, label, first};
                edges.push(e)
            }
        }
        return edges;
    }

    pub fn get_from_file_res(filename:&str,res: usize) -> Vec<Edge>
    {
        let mut set:HashSet<(usize,usize)>= HashSet::new();

        let mut edges: Vec<Edge> = vec![] ;
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
                let src:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let dst:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let label: usize = 0;
                let temp:usize = elements.next().unwrap_or("0").parse().ok().expect("malformed src");
                let first:usize = temp/res;
                if !set.contains(&(eid,first)) {
                    let e = Edge{ eid,src,dst, label, first};
                    edges.push(e);
                    set.insert((eid,first));
                }
            }
        }
        return edges;
    }

    pub fn get_src_index(edges : &Vec<Edge>) -> HashMap<usize, Vec<&Edge>>{
        let mut hash_map = HashMap::new();
        for e in edges{
            hash_map.entry(e.src).or_insert_with(Vec::new).push(e);
        }
        return hash_map
    }

    pub fn get_dst_index(edges : &Vec<Edge>) -> HashMap<usize, Vec<&Edge>>{
        let mut hash_map = HashMap::new();
        for e in edges{
            hash_map.entry(e.dst).or_insert_with(Vec::new).push(e);
        }
        return hash_map
    }

    pub fn get_srt_dst_index(edges : &Vec<Edge>) -> HashMap<(usize, usize), Edge, RandomState> {
        let mut hash_map = HashMap::new();
        for e in edges{
            hash_map.entry((e.src, e.dst)).or_insert(*e);
        }
        return hash_map
    }

    pub fn get_all_nodes(edges:&Vec<Edge>) -> Vec<usize>{
        let mut hash_map:HashMap<usize,usize> = HashMap::new();
        for e in edges {
            hash_map.entry(e.src).or_insert(0);
            hash_map.entry(e.dst).or_insert(0);
        }
        let mut keys:Vec<usize> = vec![];
        for k in hash_map.keys(){
            keys.push(*k);
        }
        keys.sort();
        keys
    }

    pub fn get_all_src_nodes(edges:&Vec<Edge>) -> Vec<usize>{
        let mut hash_map:HashMap<usize,usize> = HashMap::new();
        for e in edges {
            hash_map.entry(e.src).or_insert(0);
        }
        let mut keys:Vec<usize> = vec![];
        for k in hash_map.keys(){
            keys.push(*k);
        }
        keys.sort();
        keys
    }
    pub fn get_all_dst_nodes(edges:&Vec<Edge>) -> Vec<usize>{
        let mut hash_map:HashMap<usize,usize> = HashMap::new();
        for e in edges {
            hash_map.entry(e.dst).or_insert(0);
        }
        let mut keys:Vec<usize> = vec![];
        for k in hash_map.keys(){
            keys.push(*k);
        }
        keys.sort();
        keys
    }

}





