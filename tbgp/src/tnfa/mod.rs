use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;

use crate::join::hash_join;
use crate::matching::{Matching, PMatching};
use crate::nfa::NFA;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct TNFA{
    pub nfa : NFA,
    pub clock_set: usize,
    pub clock_cond:  (fn(usize,[usize;5],usize)-> bool,usize) //fn ( current_time, clock_val, cond)

}

#[derive(Debug)]
#[derive(Clone)]
pub struct State_Matching{
    pub mid: usize,
    pub state: usize,
    pub clock:   [usize;5],

}



impl  TNFA {
    pub fn get_from_file(filename:&str) -> Vec<TNFA>
    {
        fn all_true(current_time:usize,inp:[usize;5],cond_val:usize) -> bool {  return true}
        fn all_false(current_time:usize,inp:[usize;5],cond_val:usize) -> bool {  return false}
        fn xpass_0(current_time:usize,inp:[usize;5],cond_val:usize) -> bool {  return current_time - inp[0] < cond_val || inp[0] == 0}
        fn xpass_1(current_time:usize,inp:[usize;5],cond_val:usize) -> bool {  return current_time - inp[0] > cond_val || inp[0] == 0}



        let mut tnfa: Vec<TNFA> = vec![] ;
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
                let current_state:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let word:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let next_state:usize = elements.next().unwrap().parse().ok().expect("malformed src");
                let clock_set:usize =  elements.next().unwrap_or("0").parse().ok().expect("malformed src");
                let n= NFA {
                    current_state,
                    word,
                    next_state,
                };

                let clock_func_raw:String = elements.next().unwrap_or("true").parse().ok().expect("malformed src");
                let clock_func= match &clock_func_raw[..]{
                        "false" => all_false,
                        "x0pass" => xpass_0,
                        "x1pass" => xpass_1,
                        "true" | _ => all_true
                    };
                let clock_cond:usize = elements.next().unwrap_or("0").parse().ok().expect("malformed src");

                let t: TNFA = TNFA{nfa:n, clock_set, clock_cond: (clock_func,clock_cond) };
                tnfa.push(t)
            }
        }
        return tnfa;
    }

    pub fn apply_function<T>(current_time:usize,clocks: [usize;5] , cond: usize, func: T) -> bool where T: Fn(usize,[usize;5], usize) -> bool {
        // apply the passed function to arguments a and b
        let mut res = false;
        res = func(current_time,clocks,cond);
        return res
    }

    pub fn add_state_to_matching(matching:&Vec<Matching>) -> Vec<State_Matching> {
        matching.into_iter().map( |m| State_Matching{mid: m.mid.clone(), state: 0_usize, clock:[0,0,0,0,0]  }).collect_vec()
    }

    pub fn set_clock(clock:[usize;5], set_val:i8, current_time:usize) -> [usize;5]
    {
        if set_val > 0 {
            let mut new_clock = clock.clone();
            new_clock[set_val as usize-1] = current_time;
            return new_clock;
        }
        return clock;
    }

    pub fn set_all_clock(clock:[usize;5], number_of_clock:usize, current_time:usize) -> [usize;5]
    {
        //Used for clock experiment
        let mut new_clock = clock.clone();
        for i in 0..number_of_clock {
            new_clock[i] = current_time;
        }
        return new_clock;
    }



    pub fn apply_nfa(current_time:usize, nfa_join: &Vec<((usize, usize), TNFA )>, current_matching: &Vec<Matching>) -> Vec<Matching> {
        let first:Vec<((usize,usize),Matching)> = current_matching.iter().map(|c| ((c.state,c.word), c.clone())).collect_vec();
        let res = hash_join(&first, &nfa_join)
            .into_iter()
            .filter (|n| TNFA::apply_function(current_time ,n.0.clocks  , n.2.clock_cond.1,n.2.clock_cond.0))
            .map(|(m, (_,_), tnfa )| Matching {
                mid: m.mid,
                eid: m.eid,
                first: m.first,
                last: m.last,
                match_size: m.match_size,
                state: tnfa.nfa.next_state,
                word: m.word,
                clocks: TNFA::set_clock(m.clocks,(tnfa.clock_set as i8),current_time),
            }).collect_vec();


        return res;
    }

    pub fn apply_partial_nfa(current_time:usize,nfa_join: &Vec<((usize, usize), TNFA)>, matching: &Vec<PMatching>) -> Vec<PMatching> {
        let first = matching.iter().map(|c| ((c.state, c.word), c.clone())).collect_vec();
        let res = hash_join(&first, &nfa_join)
            .into_iter()
            .filter (|n| TNFA::apply_function(current_time ,n.0.clocks  , n.2.clock_cond.1,n.2.clock_cond.0))
            .map(|(c, (_, _), tnfa)| PMatching {
                mid: c.mid,
                eid: c.eid,
                first: c.first,
                last: c.last,
                match_size: c.match_size,
                head: c.head,
                head_idx: c.head_idx,
                tail: c.tail,
                tail_idx: c.tail_idx,
                state:tnfa.nfa.next_state,
                word: c.word,
                clocks: TNFA::set_clock(c.clocks,(tnfa.clock_set as i8),current_time),
            })
            .collect_vec();

        return res;
    }





}

