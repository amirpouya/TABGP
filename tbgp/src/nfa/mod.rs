use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use crate::active::Active;
use crate::matching::Matching;
use itertools::{ EitherOrBoth, Itertools};
use crate::join::hash_join;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct NFA {
    pub current_state: usize,
    pub word: usize,
    pub next_state: usize,
    pub accept: usize,
}




impl NFA {
    pub fn new(current_state: usize, word: usize, next_state: usize, accept: usize) -> Self {
        Self {
            current_state,
            word,
            next_state,
            accept,
        }
    }
    pub fn get_from_file(filename: &str) -> Vec<NFA>
    {
        let mut nfa: Vec<NFA> = vec![];
        let mut data_line = match File::open(&Path::new(&filename)) {
            Ok(file) => BufReader::new(file).lines(),
            Err(why) => panic!("EXCEPTION: couldn't open {}: {}",
                               Path::new(&filename).display(),
                               why.to_string(),
            )
        };
        // read the data
        for (_, line) in data_line.by_ref().enumerate() {
            let good_line = line.ok().expect("EXCEPTION: read error");
            if !good_line.starts_with('#') && good_line.len() > 0 {
                let mut elements = good_line[..].split(",");
                let current_state: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let word: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let next_state: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let accept: usize = elements.next().unwrap_or("0").parse().ok().expect("malformed src");

                let n = NFA {
                    current_state,
                    word,
                    next_state,
                    accept,
                };
                nfa.push(n)
            }
        }
        return nfa;
    }


    pub fn get_from_file_to_dic(filename: &str) -> HashMap<(usize, usize), Vec<usize>, RandomState>
    {
        //load NFA from file
        let mut nfa: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        let mut data_line = match File::open(&Path::new(&filename)) {
            Ok(file) => BufReader::new(file).lines(),
            Err(why) => panic!("EXCEPTION: couldn't open {}: {}",
                               Path::new(&filename).display(),
                               why.to_string(),
            )
        };
        // read the data
        for (_, line) in data_line.by_ref().enumerate() {
            let good_line = line.ok().expect("EXCEPTION: read error");
            if !good_line.starts_with('#') && good_line.len() > 0 {
                let mut elements = good_line[..].split(",");
                let current_state: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let word: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let next_state: usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let accept: usize = elements.next().unwrap_or("0").parse().ok().expect("malformed src");

                let n = NFA {
                    current_state,
                    word,
                    next_state,
                    accept,
                };
                nfa.entry((n.current_state, n.word)).or_insert_with(Vec::new).push(n.next_state);
            }
        }
        return nfa;
    }



    pub fn  bool_to_usize(a : [bool;5], matching_size:usize) ->usize{
    let mut out:usize = 0;
        for i in 0..matching_size  {
            let ind = 10_u32.pow((matching_size - i - 1) as u32) as usize;
            let b:usize = if a[i] {1} else {0};
            out = out + b*ind;

        }
    return out;
    }


    pub fn gen_alpha_hash(active: &Vec<Active>, matching: &Vec<Matching>, matching_size: usize) -> Vec<(usize, usize)> {
        //generate alphabet based on Actives and matching size for all the matching
        let mut a0 = matching.iter().map(|m| (m.mid, m.eid)).collect_vec();
        let mut active_pair: HashSet<usize> = HashSet::from_iter(active.iter().map(|a| a.eid.clone()));
        active_pair.remove(&0);
        let alpha = a0.into_iter().map(|(mid, eid)| (mid,
                                                     NFA::bool_to_usize([
                                                             active_pair.contains(&eid[0]),
                                                             active_pair.contains(&eid[1]),
                                                             active_pair.contains(&eid[2]),
                                                             active_pair.contains(&eid[3]),
                                                             active_pair.contains(&eid[4]),
                                                             ], matching_size))).collect_vec();



        return alpha;
    }



    pub fn gen_binary_number(number_of_digit: usize) -> Vec<usize> {
        //Generate  a list of all binary numbers based on number of digits
        if number_of_digit == 1 {
            return vec![0, 1];
        }

        let mut out: Vec<usize> = vec![];
        let dig = NFA::gen_binary_number(number_of_digit - 1);
        for d in dig
        {
            let dd = d * 10 + 0;
            out.push(dd);
            let dd = d * 10 + 1;
            out.push(dd);
        }
        out.sort();
        return out;
    }



    pub fn apply_nfa(nfa_join: &Vec<((usize, usize), (usize, usize))>, current_matching: &Vec<Matching>) -> Vec<Matching> {

        let first:Vec<((usize,usize),Matching)> = current_matching.iter().map(|c| ((c.state,c.word), c.clone())).collect_vec();
        let res = hash_join(&first, &nfa_join)
            .into_iter()
            .map(|(m, (_, _), (next_state, _))| (Matching {
                mid: m.mid,
                eid: m.eid,
                first: m.first,
                last: m.last,
                match_size: m.match_size,
                state: next_state,
                word:m.word,
                clocks:m.clocks,
            }))
            .collect_vec();
        return res;
    }


    pub fn alt_nfa_gen(nfa_size: usize, matching_size: usize, init_loop: bool, self_loop: bool) -> Vec<NFA>
    {
        let mut nfas: Vec<NFA> = vec![];
        let mut w;
        if init_loop {
            let nfa = NFA { current_state: 0, word: 0, next_state: 0, accept: 0 };
            nfas.push(nfa);
        }
        for i in 0..nfa_size {
            if self_loop {
                let nfa = NFA { current_state: i + 1, word: 0, next_state: i + 1, accept: 0 };
                nfas.push(nfa);
            }

            w = u32::pow(10, (matching_size - (i % matching_size) - 1) as u32);
            let nfa = NFA { current_state: i, word: w as usize, next_state: i + 1, accept: 0 };
            nfas.push(nfa);
        }
        w = u32::pow(10, (matching_size - (nfa_size % matching_size) - 1) as u32);
        let nfa = NFA { current_state: nfa_size, word: w as usize, next_state: 0, accept: 0 };
        nfas.push(nfa);
        return nfas;
    }


    pub fn nfa_clock_gen(clock: usize) -> Vec<NFA> {
        //Simulate a clock by adding states
        let mut nfas: Vec<NFA> = vec![];
        let nfa = NFA { current_state: 0, word: 0 as usize, next_state: 0, accept: 0 };
        nfas.push(nfa);

        let nfa = NFA { current_state: 0, word: 1 as usize, next_state: 2, accept: 0 };
        nfas.push(nfa);

        let nfa = NFA { current_state: 2, word: 10 as usize, next_state: 0, accept: 1 };
        nfas.push(nfa);
        for i in 2..clock + 2 {
            let nfa = NFA { current_state: i, word: 00 as usize, next_state: i + 1, accept: 0 };
            nfas.push(nfa);

            let nfa = NFA { current_state: i + 1, word: 10 as usize, next_state: 0, accept: 1 };
            nfas.push(nfa);
        }
        nfas
    }

}


