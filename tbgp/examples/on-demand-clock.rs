extern crate tbgp;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use itertools::__std_iter::FromIterator;

use tbgp::active::Active;
use tbgp::configs;
use tbgp::edge::Edge;
use tbgp::matching::Matching;
use tbgp::nfa::NFA;
use tbgp::tnfa::{State_Matching, TNFA};

fn main() {
    fn apply_nfa(current_time:usize, active:&HashMap<usize,HashSet<usize>>, nfa:Vec<TNFA>, m:&Vec<Matching>, pattern_size: usize) -> Vec<Matching>
    {
        if &m.len() <= &0
        {
          return vec![];
        }

        let mut current_matching :Vec<Matching> = vec![];

        let nfa_join = nfa.clone().into_iter().map(|n|((n.nfa.current_state,n.nfa.word),(n))).collect_vec();
        let td = active.keys().into_iter().sorted().collect_vec();

        for t in td {

            let mut new_matching = m.clone().into_iter().filter(|m| m.first == t.clone()).collect_vec();

            current_matching.append(&mut new_matching);
            let active_pair = active.get(&t).unwrap();
            for i in 0..current_matching.len() {
                let eid =current_matching[i].eid.clone();
                current_matching[i].word = NFA::bool_to_usize([
                                                              active_pair.contains(&eid[0]),
                                                              active_pair.contains(&eid[1]),
                                                              active_pair.contains(&eid[2]),
                                                              active_pair.contains(&eid[3]),
                                                              active_pair.contains(&eid[4]),
                                                          ], pattern_size);


            }
            current_matching = TNFA::apply_nfa(*t,&nfa_join, &current_matching);

        }
         let mut new_matching = m.clone().into_iter().filter(|m|  m.first == current_time).collect_vec();
        current_matching.append(&mut new_matching);
        return current_matching;
    }




    fn log(input:String, level:usize,DEBUG_FLAG:usize) {
        if DEBUG_FLAG >= level {
            println!("{:?}",input);
        }
    }


    let config_addr = std::env::args().nth(1).unwrap_or(("configs/simple_conf.toml").to_string()).parse::<String>().unwrap();
    let config = configs::parse(&config_addr);


    let  DEBUG_FLAG = config.debug;

    let now = Instant::now();


    let mut accepted_matching : usize = 0;

    let filename = config.input_dir.clone()+ "/edge.csv";
    let   edges = Edge::get_from_file(&filename);

    let pattern_type = config.pattern_type;
    let pattern_size = config.pattern_size;

    let active_filename = config.input_dir.clone() + "active.csv";
    let  actives = Active::get_from_file(&active_filename);


    let nfa_filename = config.nfa_dir;
    let  nfa = TNFA::get_from_file(&nfa_filename);

    let nfa_join = nfa.clone().into_iter().map(|n|((n.nfa.current_state,n.nfa.word),(n))).collect_vec();



    println!("{:?}",pattern_type.clone());



    log(format!("Edges {:?}", edges),5,DEBUG_FLAG);
    log(format!("NFA {:?}", nfa),0,DEBUG_FLAG);
    log(format!("Active {:?}", actives),5,DEBUG_FLAG);



    let matching = Matching::MatchGen(edges, &pattern_type);


    let num_matching = &matching.len();
    let matching_time = now.elapsed().as_secs_f32();

    log(format!("Matching{:?}", &matching), 5,DEBUG_FLAG);
    log(format!("Matching Size{:?}", &matching.len()),0,DEBUG_FLAG);
    log(format!("Matching Time:{}", now.elapsed().as_millis()),0,DEBUG_FLAG);

    let mut current_time = 0;
    let mut current_active:Vec<Active> = vec![];
    let mut active_hist:HashMap<usize,HashSet<usize>> = HashMap::new();
    active_hist.insert(0,HashSet::new());

    let mut all_matching:Vec<Matching> = vec![];
    let mut current_matching = matching.iter().filter(|m| m.last==current_time).cloned().collect_vec();
    //let mut current_matching = NFA::add_state_to_matching(&min_matching);
    current_matching = apply_nfa(0,&active_hist,nfa.clone(),&current_matching,pattern_size);

    for a in actives{

        if current_time < a.time{


            let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

            log(format!("current matching at {:?}: {:?}",current_time, &current_matching),5,DEBUG_FLAG);
            let  processed_matchg_count = matching.iter().filter(|m| m.last<=current_time).cloned().count();

           current_matching  = apply_nfa(current_time,&active_hist,nfa.clone(),&current_matching,pattern_size);

            log(format!("delta matching at {:?}: {:?}",&current_time, &current_matching),5,DEBUG_FLAG);

            all_matching.append(&mut current_matching.clone());
            for i in 0..all_matching.len() {
                let eid =all_matching[i].eid.clone();
                all_matching[i].word = NFA::bool_to_usize([
                                                                  active_pair.contains(&eid[0]),
                                                                  active_pair.contains(&eid[1]),
                                                                  active_pair.contains(&eid[2]),
                                                                  active_pair.contains(&eid[3]),
                                                                  active_pair.contains(&eid[4]),
                                                              ], pattern_size);



            }



            active_hist.insert(current_time,active_pair);

            all_matching = TNFA::apply_nfa(current_time,&nfa_join, &all_matching);

            log(format!("matching at {:?}: {:?}",&current_time, &all_matching),5,DEBUG_FLAG);
            log(format!("#matching at  {:?}: {:?}",&current_time, &all_matching.len()),1,DEBUG_FLAG);
            log(format!("Total Time:{},{}", &current_time,now.elapsed().as_secs_f32()),1,DEBUG_FLAG);

            log(format!("{:?},{:?},{:?}",now.elapsed(),&current_time, &processed_matchg_count),1,DEBUG_FLAG);

            current_time = a.time;
            current_active = vec![];
            current_matching = matching.iter().filter(|m| m.last==current_time).cloned().collect_vec();

            accepted_matching = accepted_matching + all_matching.iter().filter(|x| x.state == 0).count();

        }

        current_active.push(a);
    }

    //Final run for the last time point
    log(format!("delta matching at {:?}: {:?}",&current_time, &current_matching),5,DEBUG_FLAG);
    current_matching  = apply_nfa(current_time,&active_hist,nfa.clone(),&current_matching,pattern_size);
    log(format!("delta matching at {:?}: {:?}",&current_time, &current_matching),5,DEBUG_FLAG);

    let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

    all_matching.append(&mut current_matching.clone());
    for i in 0..all_matching.len() {
        let eid =all_matching[i].eid.clone();
        all_matching[i].word = NFA::bool_to_usize([
                                                      active_pair.contains(&eid[0]),
                                                      active_pair.contains(&eid[1]),
                                                      active_pair.contains(&eid[2]),
                                                      active_pair.contains(&eid[3]),
                                                      active_pair.contains(&eid[4]),
                                                  ], pattern_size);



    }
    all_matching = TNFA::apply_nfa(current_time,&nfa_join, &all_matching);
    let  processed_matchg_count = matching.iter().filter(|m| m.last<=current_time).count();
    log(format!("{:?},{:?},{:?}",now.elapsed(),&current_time, &processed_matchg_count),0,DEBUG_FLAG);

    log(format!("Total Time:{}", now.elapsed().as_millis()),0,DEBUG_FLAG);
    log(format!("Pattern_type, num_matching, matching_time, total_time"),0,DEBUG_FLAG);
    log(format!("{},{},{},{}",pattern_type.clone(),all_matching.len(),matching_time, now.elapsed().as_secs_f32()),11,DEBUG_FLAG);
    log(format!("Full Matching {:?},{:?}", now.elapsed().as_secs_f32(), all_matching), 10, DEBUG_FLAG);

    log(format!("Accepted {:?},{:?}", now.elapsed().as_secs_f32(), accepted_matching), 10, DEBUG_FLAG);



    log(format!("{:?},{:?}/{:?}", now.elapsed().as_secs_f32(), all_matching.len(),num_matching), 0, DEBUG_FLAG);

    // let mut f = File::create("od.csv").unwrap();
    // for m in all_matching{
    //    f.write(format!(" {:?}\n", m).as_ref());
    //
    // }

}

