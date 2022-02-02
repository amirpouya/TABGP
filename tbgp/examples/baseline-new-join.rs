extern crate tbgp;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use itertools::Itertools;
use itertools::__std_iter::FromIterator;

use tbgp::active::Active;
use tbgp::configs;
use tbgp::edge::Edge;
use tbgp::matching::Matching;
use tbgp::nfa::NFA;

#[derive(Debug)]
#[derive(Clone)]
pub struct NMatching {
    pub mid: usize,
    pub eid: [usize; 5],
    pub first: usize,
    pub last: usize,
    pub match_size: usize,
    pub state: Vec<usize>,
    pub word: usize,
    pub clocks: [usize; 5],
}



fn main() {
    let dedup_flag = true;
    let config_addr = std::env::args().nth(1).unwrap_or(("configs/simple_conf.toml").to_string()).parse::<String>().unwrap();

    let config = configs::parse(&config_addr);


    let  DEBUG_FLAG = config.debug;

    fn log(input:String, level:usize,DEBUG_FLAG:usize) {
        if DEBUG_FLAG >= level {
            println!("{:?}",input);
        }
    }




    let now = Instant::now();



    let filename = config.input_dir.clone()+ "/edge.csv";
    let   edges = Edge::get_from_file(&filename);

    let pattern_type = config.pattern_type;
    let pattern_size = config.pattern_size;

    let active_filename = config.input_dir.clone() + "active.csv";
    let  actives = Active::get_from_file(&active_filename);


    let nfa_filename = config.nfa_dir;
    let  nfa = NFA::get_from_file(&nfa_filename);
    //let nfa_join = nfa.into_iter().map(|n|((n.current_state,n.word),(n.next_state,n.accept))).collect_vec();

    let mut nfa_dic = HashMap::new();
    //
    let nfa_join = nfa.clone().into_iter().map(|n|((n.current_state,n.word),(n.next_state))).collect_vec();


    for (i,j) in &nfa_join {
        // nfa_dic.insert(i,j.clone());
        nfa_dic.entry(i).or_insert_with(Vec::new).push(j.clone());
    }
    println!("{:?}",pattern_type.clone());



    log(format!("Edges {:?}", edges),5,DEBUG_FLAG);
    log(format!("NFA {:?}", nfa),0,DEBUG_FLAG);
    log(format!("Active {:?}", actives),5,DEBUG_FLAG);


    let qmatching = Matching::MatchGen(edges, &pattern_type);
    let matching = qmatching.into_iter().map(|m| NMatching{
        mid: m.mid,
        eid: m.eid,
        first: m.first,
        last: m.last,
        match_size: m.match_size,
        state: vec![m.state],
        word: m.word,
        clocks: m.clocks
    }).collect_vec();
    let num_matching = matching.len();
    let matching_time = now.elapsed().as_secs_f32();

    log(format!("Matching{:?}", &matching), 5,DEBUG_FLAG);
    log(format!("Matching Size{:?}", &matching.len()),0,DEBUG_FLAG);
    log(format!("Matching Time:{}", now.elapsed().as_millis()),0,DEBUG_FLAG);

    let mut current_time = 1;
    let mut current_active:Vec<Active> = vec![];
    let mut current_matching = matching.iter().filter(|m| m.first==current_time).cloned().collect_vec();
    //let mut current_matching = NFA::add_state_to_matching(&min_matching);


    for a in actives{

        if current_time < a.time{
            let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

            //let  processed_matchg_count = matching.clone().into_iter().filter(|m| m.last<=current_time).count();

            log(format!("current matching at {:?}: {:?}",current_time, &current_matching),3,DEBUG_FLAG);
            log(format!("current active at {:?}: {:?}",current_time, &current_active),3,DEBUG_FLAG);

            apply_nfa(pattern_size, &nfa_dic, &mut current_matching, active_pair);


            current_time = a.time;
            current_active = vec![];


            let mut new_matching = matching.iter().filter(|m| m.first==current_time).cloned().collect_vec();

            current_matching.append(&mut new_matching);
            log(format!("Matching at {:?},{:?},{:?}",now.elapsed(),&current_time, &current_matching),5,DEBUG_FLAG);
            //log(format!("{:?},{:?},{:?}",now.elapsed(),&current_time, &processed_matchg_count),11,DEBUG_FLAG);
        }
        current_active.push(a);
    }



   // log(format!("{},{},{},{}",pattern_type.clone(),current_matching.len(),matching_time, now.elapsed().as_secs_f32()),0,DEBUG_FLAG);
    log(format!("{:?},{:?}/{:?}", now.elapsed().as_secs_f32(), current_matching.len(),num_matching), 0, DEBUG_FLAG);

    let fstate: usize = 2 ;
    //let unique_matching = current_matching.iter().filter(|m| m.state ==2).map(|m| m.eid.clone()).sorted_by(|m1,m2| m1.cmp(m2)).dedup().count();//.for_each(|m| println!("{:?},{:?}",m[0],m[1]));

    //println!("{:?}",unique_matching);

}

fn apply_nfa(pattern_size: usize, mut nfa_dic: &HashMap<&(usize, usize), Vec<usize>>, current_matching: &mut Vec<NMatching>, active_pair: HashSet<usize>)  {
    for i in 0..current_matching.len() {
        let mut m = &current_matching[i];
        let word = NFA::bool_to_usize([
                                          active_pair.contains(&m.eid[0]),
                                          active_pair.contains(&m.eid[1]),
                                          active_pair.contains(&m.eid[2]),
                                          active_pair.contains(&m.eid[3]),
                                          active_pair.contains(&m.eid[4]),
                                      ], pattern_size);
        let mut state = vec![];
        let mut a = m.state.clone().into_iter()
            .filter_map(|n| match nfa_dic.get(&(n, word)) {
                Some(p) => Some(p.clone()),
                None => None
            }).flatten();
        state.extend(a);
        state.dedup();
        current_matching[i].state = state;
    }
    current_matching.retain(|m| m.state.len() > 0);
}


