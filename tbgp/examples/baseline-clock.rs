extern crate tbgp;

use std::time::{Instant};


use tbgp::edge::Edge;
use tbgp::nfa::NFA;
use tbgp::active::Active;
use itertools::{Itertools};
use tbgp::matching::Matching;
use std::collections::HashSet;
use itertools::__std_iter::FromIterator;
use tbgp::configs;
use std::fs::File;
use std::io::Write;
use tbgp::tnfa::TNFA;


fn main() {

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
    let  nfa = TNFA::get_from_file(&nfa_filename);
    //let nfa_join = nfa.into_iter().map(|n|((n.current_state,n.word),(n.next_state,n.accept))).collect_vec();


    let nfa_join = nfa.clone().into_iter().map(|n|((n.nfa.current_state,n.nfa.word),(n))).collect_vec();

    println!("{:?}",pattern_type.clone());



    log(format!("Edges {:?}", edges),5,DEBUG_FLAG);
    log(format!("NFA {:?}", nfa),0,DEBUG_FLAG);
    log(format!("Active {:?}", actives),5,DEBUG_FLAG);



    let  matching = match &pattern_type[..]{
        "triangles_WOJ"=> Matching::triangles_woj(edges.clone()),
        "rectangle_woj" => Matching::rectangle_woj(edges.clone()),
        "path4" => Matching::path_four(edges.clone()),
        "path3" => Matching::path_three(edges.clone()),
        "path2"  => Matching::path_two(edges.clone()),
        "cycle_two" | _ => Matching::cycle_two(edges.clone()),

    };

    let num_matching = matching.len();
    let matching_time = now.elapsed().as_secs_f32();

    log(format!("Matching{:?}", &matching), 5,DEBUG_FLAG);
    log(format!("Matching Size{:?}", &matching.len()),0,DEBUG_FLAG);
    log(format!("Matching Time:{}", now.elapsed().as_millis()),0,DEBUG_FLAG);

    let mut current_time = 1;
    let mut current_active:Vec<Active> = vec![];
    let mut current_matching = matching.clone().into_iter().filter(|m| m.first==current_time).collect_vec();
    //let mut current_matching = NFA::add_state_to_matching(&min_matching);


    for a in actives{

        if current_time < a.time{
            let  processed_matchg_count = matching.clone().into_iter().filter(|m| m.last<=current_time).count();

            log(format!("current matching at {:?}: {:?}",current_time, &current_matching),3,DEBUG_FLAG);
            log(format!("current active at {:?}: {:?}",current_time, &current_active),3,DEBUG_FLAG);
            let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

            for i in 0..current_matching.len() {
                let eid =current_matching[i].eid.clone();
                current_matching[i].word = NFA::bool_to_usize([
                                                                  active_pair.contains(&eid[0]),
                                                                  active_pair.contains(&eid[1]),
                                                                  active_pair.contains(&eid[2]),
                                                                  active_pair.contains(&eid[3]),
                                                                  active_pair.contains(&eid[4]),
                                                              ], pattern_size);


                //let alpha = NFA::gen_alpha(&active_pair, &current_matching, pattern_size);
               // log(format!("alpha at {:?}: {:?}", &current_time, &alpha), 3, DEBUG_FLAG);
            }
            current_matching = TNFA::apply_nfa(current_time,&nfa_join,&current_matching);


            current_time = a.time;
            current_active = vec![];


            let mut new_matching = matching.clone().into_iter().filter(|m| m.first==current_time).collect_vec();

            current_matching.append(&mut new_matching);
            log(format!("Matching at {:?},{:?},{:?}",now.elapsed(),&current_time, &current_matching),5,DEBUG_FLAG);
            log(format!("{:?},{:?},{:?}",now.elapsed(),&current_time, &processed_matchg_count),11,DEBUG_FLAG);



        }
        current_active.push(a);
    }

    //Final run for the last time point
    let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

    for i in 0..current_matching.len() {
        let eid =current_matching[i].eid.clone();
        current_matching[i].word = NFA::bool_to_usize([
                                                          active_pair.contains(&eid[0]),
                                                          active_pair.contains(&eid[1]),
                                                          active_pair.contains(&eid[2]),
                                                          active_pair.contains(&eid[3]),
                                                          active_pair.contains(&eid[4]),
                                                      ], pattern_size);


        //let alpha = NFA::gen_alpha(&active_pair, &current_matching, pattern_size);
        // log(format!("alpha at {:?}: {:?}", &current_time, &alpha), 3, DEBUG_FLAG);
    }
    current_matching = TNFA::apply_nfa(current_time,&nfa_join,&current_matching);
    let processed_match = current_matching.iter().filter(|m| m.last <= current_time).count();
    log(format!("Total Time:{},{},{}", &current_time,now.elapsed().as_secs_f32(),processed_match),0,DEBUG_FLAG);

    log(format!("Total Time:{}", now.elapsed().as_millis()),0,DEBUG_FLAG);
    log(format!("Pattern_type, num_matching, matching_time, total_time"),0,DEBUG_FLAG);

    log(format!("Full Matching {:?},{:?}", now.elapsed().as_secs_f32(), current_matching), 10, DEBUG_FLAG);

    log(format!("{},{},{},{}",pattern_type.clone(),current_matching.len(),matching_time, now.elapsed().as_secs_f32()),0,DEBUG_FLAG);
    log(format!("{:?},{:?}/{:?}", now.elapsed().as_secs_f32(), current_matching.len(),num_matching), 0, DEBUG_FLAG);


    // let mut f = File::create("base.csv").unwrap();
    // for m in current_matching.into_iter().sorted_by(|m1,m2| m1.eid.cmp(&m2.eid)).collect_vec(){
    //     f.write(format!(" {:?}\n", m.eid).as_ref());
    //
    // }


}


