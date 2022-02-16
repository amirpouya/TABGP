//TODO: CLEAN


extern crate tbgp;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::ptr::hash;
use std::time::{Duration, Instant};

use array_tool::vec::Intersect;
use itertools::{EitherOrBoth, Itertools, merge_join_by};
use itertools::__std_iter::FromIterator;

use tbgp::active::Active;
use tbgp::configs;
use tbgp::edge::Edge;
use tbgp::join::hash_join;
use tbgp::matching::{Matching, PMatching};
use tbgp::nfa::NFA;

fn main() {

    let mut droped_match = 0;
    let dedup_flag = true;
    let loop_flag:usize = 0;

    
    //Set these flags based on the NFA
    let inc_flag = true; // Incremental NFA
    let dec_flag = false; // Decremental NFA
    let eq_flag = true; // Equal NFA


    let config_addr = std::env::args().nth(1).unwrap_or(("configs/simple_conf.toml").to_string()).parse::<String>().unwrap();

    let config = configs::parse(&config_addr);


    let DEBUG_FLAG = config.debug;

    fn log(input: String, level: usize, deBUG_FLAG: usize) {
        if deBUG_FLAG >= level {
            println!("{:?}", input);
        }
    }

    let now = Instant::now();


    let filename = config.input_dir.clone() + "/edge.csv";
    let edges = Edge::get_from_file(&filename);

    let pattern_type = config.pattern_type;
    let pattern_size = config.pattern_size;

    let active_filename = config.input_dir.clone() + "active.csv";
    let mut actives = Active::get_from_file(&active_filename);
    actives.push(Active{ eid: 0, time: 1000000000 });

    //actives.insert(0, Active { eid: 0, time: 0 });

    let nfa_filename = config.nfa_dir;
    let nfa = NFA::get_from_file(&nfa_filename);
    //let nfa_join = nfa.into_iter().map(|n|((n.current_state,n.word),(n.next_state,n.accept))).collect_vec();


   // let nfa = NFA::alt_nfa_gen(256, pattern_size, true, false);
    let nfa_join = nfa.clone().into_iter().map(|n| ((n.current_state, n.word), (n.next_state))).collect_vec();


    //println!("{:?}", pattern_type.clone());


    log(format!("Edges {:?}", edges), 5, DEBUG_FLAG);
    log(format!("NFA {:?}", nfa.iter().map(|n| (n.current_state, n.word, n.next_state)).collect_vec()), 1,DEBUG_FLAG);
    log(format!("Active {:?}", actives), 5, DEBUG_FLAG);


    let mut matching = vec![PMatching {
        mid: 0,
        eid: [0, 0, 0, 0, 0],
        first: 0,
        last: 0,
        match_size: 0,
        head: 0,
        head_idx: 1,
        tail: 0,
        tail_idx: 0,
        state: 0,
        word: 0,
        clocks:[0,0,0,0,0]
    }];

    let mut dic_state: HashMap<[usize; 5], Vec<usize>> = HashMap::new();
    dic_state.insert([0, 0, 0, 0, 0], vec![0]);

    let mut mid_counter: usize = 0;
    let mut full_mid_counter = 0;

    let mut current_active: Vec<Active> = vec![];
    let mut current_time = 1;


    let  mut c = 0;
    let mut a = actives[0];
    while c <  actives.len() || current_active.len() > 0 {
        if c < actives.len(){
            a = actives[c];
            c=c+1;
        }
        let mut path1_matching: Vec<PMatching> = vec![];
        if current_time < a.time {

            let current_edge = edges.iter().filter(|e| e.first == current_time);
            let  active_pair: HashSet<usize> = HashSet::from_iter(current_active.iter().map(|a| a.eid.clone()));

            let mut path2_matching_forward: Vec<PMatching> = vec![];
            let mut path2_matching_reverse: Vec<PMatching> = vec![];
            let mut path2_matching_eq: Vec<PMatching> = vec![];


            if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                //Add new edges to the matching
                let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();

                current_edge.clone().for_each(|e| {
                    for s in empty_state.iter() {
                        if inc_flag {
                            let i = 0;
                            mid_counter += 1;
                            dic_state.insert(PMatching::fill_array(e.eid, i, [0, 0, 0, 0, 0]), empty_state.clone());
                            path1_matching.push(PMatching {
                                mid: mid_counter,
                                eid: PMatching::fill_array(e.eid, i, [0, 0, 0, 0, 0]),
                                first: e.first,
                                last: e.first,
                                match_size: 1,
                                head:  e.src  ,
                                head_idx: if i > 0 { i as i8 } else { -1 },
                                tail: e.dst,
                                tail_idx: if i < (pattern_size - 1) as i8 { i } else { -1 },
                                state: s.clone(),
                                word: 10_u32.pow((pattern_size - i as usize - 1) as u32) as usize,
                                clocks: [0,0,0,0,0]
                            });
                        } else if dec_flag {
                            let i = pattern_size - 1;
                            mid_counter += 1;
                            dic_state.insert(PMatching::fill_array(e.eid, i as i8, [0, 0, 0, 0, 0]), empty_state.clone());
                            path1_matching.push(PMatching {
                                mid: mid_counter,
                                eid: PMatching::fill_array(e.eid, i as i8, [0, 0, 0, 0, 0]),
                                first: e.first,
                                last: e.first,
                                match_size: 1,
                                head: e.src,
                                head_idx: if i > 0 { i as i8 } else { -1 },
                                tail: e.dst,
                                tail_idx: if i < (pattern_size - 1) { i as i8} else { -1 } ,
                                state: s.clone(),
                                word: 10_u32.pow((pattern_size - i - 1) as u32) as usize,
                                clocks: [0,0,0,0,0]

                            });
                        } else {
                            for i in 0..pattern_size {
                                mid_counter += 1;
                                dic_state.insert(PMatching::fill_array(e.eid, i as i8, [0, 0, 0, 0, 0]), empty_state.clone());
                                path1_matching.push(PMatching {
                                    mid: mid_counter,
                                    eid: PMatching::fill_array(e.eid, i as i8, [0, 0, 0, 0, 0]),
                                    first: e.first,
                                    last: e.first,
                                    match_size: 1,
                                    head: e.src,
                                    head_idx: if i > 0 { i as i8 } else { -1 },
                                    tail: e.dst,
                                    tail_idx: if i < (pattern_size - 1) { i as i8} else { -1 } ,
                                    state: s.clone(),
                                    word: 10_u32.pow((pattern_size - i - 1) as u32) as usize,
                                    clocks: [0,0,0,0,0]

                                });
                            }
                        }
                    }
                });
            }
            full_mid_counter = mid_counter;
            log(format!("Path1 at time {:?},{:?}", &current_time, &path1_matching), 5, DEBUG_FLAG);


            //Path 2

            //E


            //E size one
            let matching_head = matching.iter().filter(|m| m.head_idx >= 0 && m.match_size == 1).map(|m| (m.head, m.clone())).collect_vec();
            let matching_tail = matching.iter().filter(|m| m.tail_idx >= 0 && m.match_size == 1).map(|m| (m.tail, m.clone())).collect_vec();


            //Delta E
            let edge_head = current_edge.clone().map(|e| (e.src, e.clone())).collect_vec();
            let edge_tail = current_edge.clone().map(|e| (e.dst, e.clone())).collect_vec();
            let edge_head_tail = current_edge.clone().map(|e| ((e.dst,e.src), e.clone())).collect_vec();

            let dede = hash_join(&edge_tail, &edge_head);

            let mut dedede = vec![];

            if pattern_size > 2 || pattern_type == "path2" {
                if !dec_flag {
                    path2_matching_forward = hash_join(&matching_tail, &edge_head)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, (e.tail_idx + 1) as usize as i8, e.eid), first: e.first, last: de.first, match_size: 2, head: e.head, head_idx: e.head_idx, tail: de.dst, tail_idx: if e.tail_idx + 1 >= (pattern_size - 1) as i8 { -1 } else { e.tail_idx + 1 }, state: e.state, word: 0 ,clocks: [0,0,0,0,0]
                        })
                        .collect_vec();

                    for i in 0..path2_matching_forward.len()
                    {
                        let eid = path2_matching_forward[i].eid.clone();
                        path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                active_pair.contains(&eid[3]),
                                                                                active_pair.contains(&eid[4]),
                                                                            ], pattern_size);
                    }

                    mid_counter += path2_matching_forward.len();
                    log(format!("Path2F at time {:?},{:?}", &current_time, &path2_matching_forward), 5, DEBUG_FLAG);
                }

                if !inc_flag {

                    path2_matching_reverse = hash_join(&matching_head, &edge_tail.clone())
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching {
                            mid: i + 1 + mid_counter,
                            eid: PMatching::fill_array(de.eid, e.head_idx - 1, e.eid),
                            first: e.first,
                            last: de.first,
                            match_size: 2,
                            head: de.src,
                            head_idx: if e.head_idx - 1 > 0
                            { e.head_idx - 1 } else { -1 },
                            tail: e.tail,
                            tail_idx: e.tail_idx,
                            state: e.state,
                            word: 0,
                            clocks: [0,0,0,0,0]
                        })
                        .collect_vec();


                    for i in 0..path2_matching_reverse.len()
                    {
                        let eid = path2_matching_reverse[i].eid.clone();
                        path2_matching_reverse[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                active_pair.contains(&eid[3]),
                                                                                active_pair.contains(&eid[4]),
                                                                            ], pattern_size);
                    }


                    mid_counter += path2_matching_reverse.len();

                    full_mid_counter = mid_counter - full_mid_counter;
                    log(format!("Path2R at time {:?},{:?}", &current_time, &path2_matching_reverse), 5, DEBUG_FLAG);
                }

                if eq_flag || pattern_size > 2 {
                    if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                        let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                        dede.iter()
                            .for_each(|(e1, _, e2)| {
                                for s in empty_state.iter() {
                                    for i in 0..pattern_size - 1 {
                                        mid_counter += 1;
                                        path2_matching_eq.push(PMatching {
                                            mid: mid_counter,
                                            eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid], i, [0, 0, 0, 0, 0]),
                                            first: e1.first,
                                            last: e1.first,
                                            match_size: 2,
                                            head: e1.src,
                                            head_idx: if i > 0 { i as i8 } else { -1 as i8 },
                                            tail: e2.dst,
                                            tail_idx: if i < pattern_size - 2 { i as i8 + 1 } else { -1 },
                                            state: s.clone(),
                                            word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32)) as usize,
                                            clocks: [0,0,0,0,0]
                                        });
                                    }
                                }
                            });
                    }
                    log(format!("Path2 at time {:?},{:?}", &current_time, &path2_matching_eq), 5, DEBUG_FLAG);
                    full_mid_counter = mid_counter - full_mid_counter;
                }
            }

            //Cycle 2
            if  pattern_type == "cycle_two" {
                let matching_head_tail = matching.iter().map(|m| ((m.head, m.tail), m.clone())).collect_vec();

                path2_matching_forward = hash_join(&matching_head_tail, &edge_head_tail)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, (e.tail_idx + 1) as usize as i8, e.eid), first: e.first, last: de.first, match_size: 2, head: e.head, head_idx: e.head_idx, tail: de.dst, tail_idx: if e.tail_idx + 1 >= (pattern_size - 1) as i8 { -1 } else { e.tail_idx + 1 }, state: e.state, word: 0, clocks: [0,0,0,0,0]})
                        //.filter(|m| m.head == m.tail)
                        .collect_vec();

                    for i in 0..path2_matching_forward.len()
                    {
                        let eid = path2_matching_forward[i].eid.clone();
                        path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                active_pair.contains(&eid[3]),
                                                                                active_pair.contains(&eid[4]),
                                                                            ], pattern_size);
                    }

                    mid_counter += path2_matching_forward.len();
                    log(format!("Path2F at time {:?},{:?}", &current_time, &path2_matching_forward), 5, DEBUG_FLAG);



                if eq_flag  {
                    if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                        let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                        dede.iter().filter(|(e1,_,e2)| e1.src == e2.dst)
                            .for_each(|(e1, _, e2)| {
                                for s in empty_state.iter() {
                                    for i in 0..pattern_size - 1 {
                                        mid_counter += 1;
                                        path2_matching_eq.push(PMatching {
                                            mid: mid_counter,
                                            eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid], i, [0, 0, 0, 0, 0]),
                                            first: e1.first,
                                            last: e1.first,
                                            match_size: 2,
                                            head: e1.src,
                                            head_idx: if i > 0 { i as i8 } else { -1 as i8 },
                                            tail: e2.dst,
                                            tail_idx: if i < pattern_size - 2 { i as i8 + 1 } else { -1 },
                                            state: s.clone(),
                                            word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32)) as usize,
                                            clocks: [0,0,0,0,0]
                                        });
                                    }
                                }
                            });
                    }
                    log(format!("Path2 at time {:?},{:?}", &current_time, &path2_matching_eq), 5, DEBUG_FLAG);
                    full_mid_counter = mid_counter - full_mid_counter;
                }
            }


            //End of Path2
            // Path3

            //E size two
            let matching_head_2 = matching.iter().filter(|m| m.head_idx >= 0 && m.match_size == 2).map(|m| (m.head, m.clone()));
            let matching_tail_2 = matching.iter().filter(|m| m.tail_idx >= 0 && m.match_size == 2).map(|m| (m.tail, m.clone()));
            let matching_head_tail = matching.iter().filter(|m|  m.match_size == 2).map(|m| ((m.head, m.tail), m.clone())).collect_vec();


            let mut path3_matching_forward = vec![];
            let mut path3_matching_reverse = vec![];
            let mut path3_matching_eq = vec![];
            let mut path1_path2_matching_forward = vec![];
            let mut path2_path1_matching_forward = vec![];


            if pattern_size > 3 || pattern_type == "path3" {


                // E E de
                if !dec_flag {
                    path3_matching_forward = hash_join(&matching_tail_2.clone().collect_vec(), &edge_head)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.tail_idx + 1, e.eid), first: e.first, last: de.first, match_size: 3, head: e.head, head_idx: e.head_idx, tail: de.dst, tail_idx: if e.tail_idx + 1 > (pattern_size - 1) as i8 { 0 } else { e.tail_idx + 1 }, state: e.state, word: 0,clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path3_matching_forward.len()
                    {
                        let eid = path3_matching_forward[i].eid.clone();
                        path3_matching_forward[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                true,
                                                                                false,
                                                                                false,
                                                                            ], pattern_size);
                    }

                    mid_counter += path3_matching_forward.len();


                    //E de de
                    let a = dede.iter().map(|(e1, _, e2)| (e1.src, (e1.clone(), e2.clone()))).collect_vec();
                    let ee = matching_tail.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0  && e.tail_idx + 2 < pattern_size as i8)).collect_vec();
                    path1_path2_matching_forward = hash_join(&ee, &a)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.tail_idx+1 ) as usize, e.eid), first: e.first, last: de1.first, match_size: 3, head: e.head, head_idx: e.head_idx, tail:de2.dst, tail_idx:  if e.tail_idx + 3 < pattern_size as i8 { e.tail_idx + 2} else {-1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path1_path2_matching_forward.len()
                    {
                        let eid = path1_path2_matching_forward[i].eid.clone();
                        path1_path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                      active_pair.contains(&eid[0]),
                                                                                      true,
                                                                                      true,
                                                                                      false,
                                                                                      false,
                                                                                  ], pattern_size);
                    }

                    mid_counter += path1_path2_matching_forward.len();


                    log(format!("Path3 at time {:?},{:?}", &current_time, &path3_matching_forward), 5, DEBUG_FLAG);
                    log(format!("Path1-2 at time {:?},{:?}", &current_time, &path1_path2_matching_forward), 5, DEBUG_FLAG);
                }

                // de E E
                if !inc_flag {

                    //log(format!("E {:?},{:?}", &current_time, &matching_head.clone().collect_vec()), 5, deBUG_FLAG);
                    //log(format!("de {:?},{:?}", &current_time, &edge_tail.clone().collect_vec()), 5, deBUG_FLAG);

                    path3_matching_reverse = hash_join(&matching_head_2.clone().collect_vec(), &edge_tail.clone())
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.head_idx-1, e.eid), first: e.first, last: de.first, match_size: 3, head: de.src, head_idx: if e.head_idx - 1 > 0 { e.head_idx-1 } else { -1 }, tail: e.tail, tail_idx: e.tail_idx, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();


                    for i in 0..path3_matching_reverse.len()
                    {
                        let eid = path3_matching_reverse[i].eid.clone();
                        path3_matching_reverse[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                active_pair.contains(&eid[3]),
                                                                                active_pair.contains(&eid[4]),
                                                                            ], pattern_size);
                    }


                    mid_counter += path3_matching_reverse.len();

                    //de de E
                    let a = dede.iter().map(|(e1, _, e2)| (e2.dst, (e1.clone(), e2.clone()))).collect_vec();
                    let ee = matching_head.clone().into_iter().filter(|(_, e)| (e.head_idx - 1 > 0)).collect_vec();
                    path2_path1_matching_forward = hash_join(&ee, &a)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.head_idx - 2) as usize, e.eid), first: e.first, last: de1.first, match_size: 3, head: de1.src, head_idx:if e.head_idx - 2 > 0 { e.head_idx-2} else { -1 }, tail: e.tail, tail_idx: e.tail_idx, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path2_path1_matching_forward.len()
                    {
                        let eid = path2_path1_matching_forward[i].eid.clone();
                        path2_path1_matching_forward[i].word = NFA::bool_to_usize([
                                                                                      active_pair.contains(&eid[0]),
                                                                                      active_pair.contains(&eid[1]),
                                                                                      active_pair.contains(&eid[2]),
                                                                                      active_pair.contains(&eid[3]),
                                                                                      active_pair.contains(&eid[4]),
                                                                                  ], pattern_size);
                    }

                    mid_counter += path2_path1_matching_forward.len();


                    log(format!("Path3 at time {:?},{:?}", &current_time, &path3_matching_reverse), 5, DEBUG_FLAG);
                    log(format!("Path2-1 at time {:?},{:?}", &current_time, &path2_path1_matching_forward), 5, DEBUG_FLAG);
                }


                //de de de
                if eq_flag || pattern_size > 3 {
                    let a = dede.iter().map(|(e1, _, e2)| (e2.dst, (e1.clone(), e2.clone()))).collect_vec();
                    dedede = hash_join(&edge_head, &a).iter().map(|(e3, usize, (e1, e2))| (e1.clone(), e2.clone(), e3.clone())).collect_vec();

                    if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                        let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                        dedede.iter()
                            .for_each(|(e1, e2, e3)| {
                                for s in empty_state.iter() {
                                    for i in 0..pattern_size - 2 {

                                        mid_counter += 1;
                                        path3_matching_eq.push(PMatching {
                                            mid: mid_counter,
                                            eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid, e3.eid], i, [0, 0, 0, 0, 0]),
                                            first: e1.first,
                                            last: e1.first,
                                            match_size: 3,
                                            head: e1.src,
                                            head_idx: if i > 0 { i as i8 } else { -1 } ,
                                            tail:e3.dst ,
                                            tail_idx:  if i < pattern_size - 3 { (i + 2) as i8} else { -1 } ,
                                            state: s.clone(),
                                            word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32) + 10_u32.pow((pattern_size - i - 3) as u32)) as usize,
                                            clocks: [0,0,0,0,0]
                                        });
                                    }
                                }
                            });
                    }
                    log(format!("Path3 at time {:?},{:?}", &current_time, &path3_matching_eq), 5, DEBUG_FLAG);
                }

                full_mid_counter = mid_counter - full_mid_counter;

            }

            if  pattern_type == "triangles_WOJ"
            {// E E de
                //let matching_head_tail = matching_head_2.map(|(_,m)|((m.head, m.tail), m.clone())).collect_vec();

                path3_matching_forward = hash_join(&matching_head_tail, &edge_head_tail)
                    .iter()
                    .enumerate()
                    .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.tail_idx + 1, e.eid), first: e.first, last: de.first, match_size: 3, head: e.head, head_idx: e.head_idx, tail:  { de.dst }, tail_idx: if e.tail_idx + 1 > (pattern_size - 1) as i8 { 0 } else {e.tail_idx + 1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                    .collect_vec();

                for i in 0..path3_matching_forward.len()
                {
                    let eid = path3_matching_forward[i].eid.clone();
                    path3_matching_forward[i].word = NFA::bool_to_usize([
                                                                            active_pair.contains(&eid[0]),
                                                                            active_pair.contains(&eid[1]),
                                                                            true,
                                                                            false,
                                                                            false,
                                                                        ], pattern_size);
                }

                mid_counter += path3_matching_forward.len();


                //E de de
                let a = dede.iter().map(|(e1, _, e2)| ((e1.src,e2.dst), (e1.clone(), e2.clone()))).collect_vec();
                let ee = matching_head_tail.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0 && e.tail_idx + 2 < pattern_size as i8)).collect_vec();
                path1_path2_matching_forward = hash_join(&ee, &a)
                    .iter()
                    .enumerate()
                    .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.tail_idx + 1) as usize, e.eid), first: e.first, last: de1.first, match_size: 3, head: e.head, head_idx: e.head_idx, tail: de2.dst , tail_idx: if e.tail_idx + 3 < pattern_size as i8 { e.tail_idx + 2 } else { -1 }, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                    .collect_vec();

                for i in 0..path1_path2_matching_forward.len()
                {
                    let eid = path1_path2_matching_forward[i].eid.clone();
                    path1_path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                  active_pair.contains(&eid[0]),
                                                                                  true,
                                                                                  true,
                                                                                  false,
                                                                                  false,
                                                                              ], pattern_size);
                }

                mid_counter += path1_path2_matching_forward.len();


                log(format!("TRG at time {:?},{:?}", &current_time, &path3_matching_forward), 5, DEBUG_FLAG);
                log(format!("TRG1-2 at time {:?},{:?}", &current_time, &path1_path2_matching_forward), 5, DEBUG_FLAG);



            //de de de
            if eq_flag {
                let a = dede.iter().map(|(e1, _, e2)| ((e1.src,e2.dst), (e1.clone(), e2.clone()))).collect_vec();
                let dedede = hash_join(&edge_head_tail, &a).iter().map(|(e3, usize, (e1, e2))| (e1.clone(), e2.clone(), e3.clone())).collect_vec();

                if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                    let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                    dedede.iter()
                        .for_each(|(e1, e2, e3)| {
                            for s in empty_state.iter() {
                                for i in 0..pattern_size - 1 {
                                    mid_counter += 1;
                                    path3_matching_eq.push(PMatching {
                                        mid: mid_counter,
                                        eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid, e3.eid], i, [0, 0, 0, 0, 0]),
                                        first: e1.first,
                                        last: e1.first,
                                        match_size: 3,
                                        head: e1.src ,
                                        head_idx: if i > 0 { i as i8 } else { -1 } ,
                                        tail: e3.dst,
                                        tail_idx: if i < pattern_size - 3 { i  as i8 + 2  } else { -1 },
                                        state: s.clone(),
                                        //word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32) + 10_u32.pow((pattern_size - i - 3) as u32)) as usize,
                                        word: 111,
                                        clocks: [0,0,0,0,0]
                                    });
                                }
                            }
                        });
                }
                log(format!("TRG at time {:?},{:?}", &current_time, &path3_matching_eq), 5, DEBUG_FLAG);
            }

                full_mid_counter = mid_counter - full_mid_counter;


            }

            //End of Path3 and Trg

            let matching_head_3 = matching.iter().filter(|m| m.head_idx >= 0 && m.match_size == 3).map(|m| (m.head, m.clone()));
            let matching_tail_3 = matching.iter().filter(|m| m.tail_idx >= 0 && m.match_size == 3).map(|m| (m.tail, m.clone()));
            let matching_head_tail3 = matching.iter().filter(|m|  m.match_size == 3).map(|m| ((m.head, m.tail), m.clone())).collect_vec();


            let mut path4_matching_forward = vec![];
            let mut path4_matching_reverse = vec![];
            let mut path2_path2_matching_forward = vec ![];
            let mut path2_path2_matching_reverse = vec![];
            let mut path3_path1_matching_forward = vec![];


            let mut path4_matching_eq = vec![];
            //let mut path1_path2_matching_forward = vec![];
            //let mut path2_path1_matching_forward = vec![];


            if pattern_size > 4 || pattern_type == "path4" {


                //E E E de
                if !dec_flag {
                    path4_matching_forward = hash_join(&matching_tail_3.clone().collect_vec(), &edge_head)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.tail_idx + 1, e.eid), first: e.first, last: de.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail: de.dst, tail_idx: if e.tail_idx + 1 > (pattern_size - 1) as i8 { 0 } else { e.tail_idx + 1 }, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path4_matching_forward.len()
                    {
                        let eid = path4_matching_forward[i].eid.clone();
                        path4_matching_forward[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                true,
                                                                                false,
                                                                            ], pattern_size);
                    }

                    mid_counter += path4_matching_forward.len();


                    //E E de de
                    let a = dede.iter().map(|(e1, _, e2)| (e1.src, (e1.clone(), e2.clone()))).collect_vec();
                    let eee = matching_tail_2.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0  && e.tail_idx + 2 < pattern_size as i8)).collect_vec();
                    path2_path2_matching_forward = hash_join(&eee, &a)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.tail_idx + 1) as usize, e.eid), first: e.first, last: de1.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail:de2.dst, tail_idx:  if e.tail_idx + 3 < pattern_size as i8 { e.tail_idx + 2} else {-1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path2_path2_matching_forward.len()
                    {
                        let eid = path2_path2_matching_forward[i].eid.clone();
                        path2_path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                      active_pair.contains(&eid[0]),
                                                                                      active_pair.contains(&eid[0]),
                                                                                      true,
                                                                                      true,
                                                                                      false,

                                                                                  ], pattern_size);
                    }

                    mid_counter += path2_path2_matching_forward.len();


                    //E de de de  //TODO: Impelement
                    let a = dedede.iter().map(|(e1, e2, e3)| (e1.src, (e1.clone(), e2.clone(),e3.clone()))).collect_vec();
                    let eee = matching_tail.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0  && e.tail_idx + 3 < pattern_size as i8)).collect_vec();
                    path3_path1_matching_forward = hash_join(&eee, &a)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, (de1,de2, de3))): (usize, &(PMatching, _, (Edge, Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid, de3.eid], (e.tail_idx + 1) as usize, e.eid), first: e.first, last: de1.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail:de2.dst, tail_idx:  if e.tail_idx + 4 < pattern_size as i8 { e.tail_idx + 3} else {-1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path3_path1_matching_forward.len()
                    {
                        let eid = path3_path1_matching_forward[i].eid.clone();
                        path3_path1_matching_forward[i].word = NFA::bool_to_usize([

                                                                                      active_pair.contains(&eid[0]),
                                                                                      true,
                                                                                      true,
                                                                                      true,
                                                                                      false,

                                                                                  ], pattern_size);
                    }


                    log(format!("Path4 at time {:?},{:?}", &current_time, &path4_matching_forward), 5, DEBUG_FLAG);
                    log(format!("Path2-2-f at time {:?},{:?}", &current_time, &path2_path2_matching_forward), 5, DEBUG_FLAG);
                    log(format!("Path3-1-f at time {:?},{:?}", &current_time, &path3_path1_matching_forward), 5, DEBUG_FLAG);

                }

                // de E E E
                if !inc_flag {

                    //log(format!("E {:?},{:?}", &current_time, &matching_head.clone().collect_vec()), 5, deBUG_FLAG);
                    //log(format!("de {:?},{:?}", &current_time, &edge_tail.clone().collect_vec()), 5, deBUG_FLAG);

                    path4_matching_reverse = hash_join(&matching_head_3.clone().collect_vec(), &edge_tail.clone())
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.head_idx-1, e.eid), first: e.first, last: de.first, match_size: 4, head: de.src, head_idx: if e.head_idx - 1 > 0 { e.head_idx-1 } else { -1 }, tail: e.tail, tail_idx: e.tail_idx, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();


                    for i in 0..path4_matching_reverse.len()
                    {
                        let eid = path4_matching_reverse[i].eid.clone();
                        path4_matching_reverse[i].word = NFA::bool_to_usize([
                                                                                active_pair.contains(&eid[0]),
                                                                                active_pair.contains(&eid[1]),
                                                                                active_pair.contains(&eid[2]),
                                                                                active_pair.contains(&eid[3]),
                                                                                active_pair.contains(&eid[4]),
                                                                            ], pattern_size);
                    }


                    mid_counter += path4_matching_reverse.len();

                    //de de E E
                    let a = dede.iter().map(|(e1, _, e2)| (e2.dst, (e1.clone(), e2.clone()))).collect_vec();
                    let eee = matching_head_2.clone().into_iter().filter(|(_, e)| (e.head_idx - 1 > 0)).collect_vec();
                    path2_path2_matching_reverse = hash_join(&eee, &a)
                        .iter()
                        .enumerate()
                        .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.head_idx - 2) as usize, e.eid), first: e.first, last: de1.first, match_size: 4, head: de1.src, head_idx:if e.head_idx - 2 > 0 { e.head_idx-2} else { -1 }, tail: e.tail, tail_idx: e.tail_idx, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                        .collect_vec();

                    for i in 0..path2_path2_matching_reverse.len()
                    {
                        let eid = path2_path2_matching_reverse[i].eid.clone();
                        path2_path2_matching_reverse[i].word = NFA::bool_to_usize([
                                                                                      active_pair.contains(&eid[0]),
                                                                                      active_pair.contains(&eid[1]),
                                                                                      active_pair.contains(&eid[2]),
                                                                                      active_pair.contains(&eid[3]),
                                                                                      active_pair.contains(&eid[4]),
                                                                                  ], pattern_size);
                    }

                    mid_counter += path2_path2_matching_reverse.len();

                    // de de de E

                    log(format!("Path4 at time {:?},{:?}", &current_time, &path4_matching_reverse), 5, DEBUG_FLAG);
                    log(format!("Path2-2-r at time {:?},{:?}", &current_time, &path2_path2_matching_reverse), 5, DEBUG_FLAG);
                    full_mid_counter = mid_counter - full_mid_counter;

                }


                //de de de
                if eq_flag {
                    let a = dede.iter().map(|(e1, _, e2)| (e2.dst, (e1.clone(), e2.clone()))).collect_vec();
                    let dedede = hash_join(&edge_head, &a).iter().map(|(e3, usize, (e1, e2))| (e1.clone(), e2.clone(), e3.clone())).collect_vec();

                    if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                        let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                        dedede.iter()
                            .for_each(|(e1, e2, e3)| {
                                for s in empty_state.iter() {
                                    for i in 0..pattern_size - 2 {

                                        mid_counter += 1;
                                        path3_matching_eq.push(PMatching {
                                            mid: mid_counter,
                                            eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid, e3.eid], i, [0, 0, 0, 0, 0]),
                                            first: e1.first,
                                            last: e1.first,
                                            match_size: 3,
                                            head: e1.src,
                                            head_idx: if i > 0 { i as i8 } else { -1 } ,
                                            tail:e3.dst ,
                                            tail_idx:  if i < pattern_size - 3 { (i + 2) as i8} else { -1 } ,
                                            state: s.clone(),
                                            word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32) + 10_u32.pow((pattern_size - i - 3) as u32)) as usize,
                                            clocks: [0,0,0,0,0]
                                        });
                                    }
                                }
                            });
                    }
                    log(format!("Path3 at time {:?},{:?}", &current_time, &path3_matching_eq), 5, DEBUG_FLAG);
                }


                let p2 = dede.iter().map(|(e1, _, e2)| PMatching {
                    mid: 0,
                    eid: [e1.eid, e2.eid, 0, 0, 0],
                    first: e1.first,
                    last: e2.first,
                    match_size: 2,
                    head: e1.src,
                    head_idx: 0,
                    tail: e2.dst,
                    tail_idx: 1,
                    state: 0,
                    word: 0,
                    clocks: [0,0,0,0,0]
                });
            }


            if  pattern_type == "rectangle_woj"
            {// E E E de
                

                path4_matching_forward = hash_join(&matching_head_tail3.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0)).collect_vec(), &edge_head_tail)
                    .iter()
                    .enumerate()
                    .map(|(i, (e, _, de)): (usize, &(PMatching, _, Edge))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array(de.eid, e.tail_idx + 1, e.eid), first: e.first, last: de.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail:  { de.dst }, tail_idx: if e.tail_idx + 1 > (pattern_size - 1) as i8 { 0 } else {e.tail_idx + 1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                    .collect_vec();

                for i in 0..path4_matching_forward.len()
                {
                    let eid = path4_matching_forward[i].eid.clone();
                    path4_matching_forward[i].word = NFA::bool_to_usize([
                                                                            active_pair.contains(&eid[0]),
                                                                            active_pair.contains(&eid[1]),
                                                                            active_pair.contains(&eid[2]),
                                                                            true,
                                                                            false,

                                                                        ], pattern_size);
                }

                mid_counter += path4_matching_forward.len();

                //E E de de
                let a = dede.iter().map(|(e1, _, e2)| ((e1.src,e2.dst), (e1.clone(), e2.clone()))).collect_vec();
                let eee = matching_head_tail3.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0  && e.tail_idx + 2 < pattern_size as i8)).collect_vec();
                path2_path2_matching_forward = hash_join(&eee, &a)
                    .iter()
                    .enumerate()
                    .map(|(i, (e, _, (de1, de2))): (usize, &(PMatching, _, (Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid], (e.tail_idx + 1) as usize, e.eid), first: e.first, last: de1.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail:de2.dst, tail_idx:  if e.tail_idx + 3 < pattern_size as i8 { e.tail_idx + 2} else {-1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                    .collect_vec();

                for i in 0..path2_path2_matching_forward.len()
                {
                    let eid = path2_path2_matching_forward[i].eid.clone();
                    path2_path2_matching_forward[i].word = NFA::bool_to_usize([
                                                                                  active_pair.contains(&eid[0]),
                                                                                  active_pair.contains(&eid[0]),
                                                                                  true,
                                                                                  true,
                                                                                  false,

                                                                              ], pattern_size);
                }

                mid_counter += path2_path2_matching_forward.len();


                //E de de de
                let a = dedede.iter().map(|(e1, e2, e3)| ((e1.src,e3.dst), (e1.clone(), e2.clone(),e3.clone()))).collect_vec();
                let ee = matching_head_tail.clone().into_iter().filter(|(_, e)| (e.tail_idx >= 0 && e.tail_idx + 2 < pattern_size as i8)).collect_vec();
                path4_matching_reverse = hash_join(&ee, &a)
                    .iter()
                    .enumerate()
                    .map(|(i, (e, _, (de1,de2, de3))): (usize, &(PMatching, _, (Edge, Edge, Edge)))| PMatching { mid: i + 1 + mid_counter, eid: PMatching::fill_array_with_vec(vec![de1.eid, de2.eid, de3.eid], (e.tail_idx + 1) as usize, e.eid), first: e.first, last: de1.first, match_size: 4, head: e.head, head_idx: e.head_idx, tail:de2.dst, tail_idx:  if e.tail_idx + 4 < pattern_size as i8 { e.tail_idx + 3} else {-1}, state: e.state, word: 0, clocks: [0,0,0,0,0] })
                    .collect_vec();

                for i in 0..path4_matching_reverse.len()
                {
                    let eid = path4_matching_reverse[i].eid.clone();
                    path4_matching_reverse[i].word = NFA::bool_to_usize([
                                                                                  active_pair.contains(&eid[0]),
                                                                                  true,
                                                                                  true,
                                                                                  false,
                                                                                  false,
                                                                              ], pattern_size);
                }

                mid_counter += path4_matching_reverse.len();





                //de de de de
                if eq_flag {
                    let a = dedede.iter().map(|(e1, e2, e3)| ((e1.src,e3.dst), (e1.clone(),e2.clone(), e3.clone()))).collect_vec();
                    let dededede = hash_join(&edge_head_tail, &a).iter().map(|(e4, usize, (e1, e2,e3))| (e1.clone(), e2.clone(), e3.clone(),e4.clone())).collect_vec();

                    if dic_state.contains_key(&[0, 0, 0, 0, 0]) {
                        let empty_state = dic_state.get(&[0, 0, 0, 0, 0]).unwrap().clone();
                        dededede.iter()
                            .for_each(|(e1, e2, e3,e4)| {
                                for s in empty_state.iter() {
                                    for i in 0..pattern_size - 1 {
                                        mid_counter += 1;
                                        path4_matching_eq.push(PMatching {
                                            mid: mid_counter,
                                            eid: PMatching::fill_array_with_vec(vec![e1.eid, e2.eid, e3.eid,e4.eid], i, [0, 0, 0, 0, 0]),
                                            first: e1.first,
                                            last: e1.first,
                                            match_size: 4,
                                            head: e1.src ,
                                            head_idx: if i > 0 { i as i8 } else { -1 } ,
                                            tail: e3.dst,
                                            tail_idx: if i < pattern_size - 3 { i  as i8 + 2  } else { -1 },
                                            state: s.clone(),
                                            word: (10_u32.pow((pattern_size - i - 1) as u32) + 10_u32.pow((pattern_size - i - 2) as u32) + 10_u32.pow((pattern_size - i - 3) as u32)+ 10_u32.pow((pattern_size - i - 4) as u32)) as usize,
                                            clocks: [0,0,0,0,0]
                                        });
                                    }
                                }
                            });
                    }
                    log(format!("TRG at time {:?},{:?}", &current_time, &path3_matching_eq), 5, DEBUG_FLAG);
                }

            }


            for i in 0..matching.len()
            {
                let eid = matching[i].eid.clone();
                matching[i].word = NFA::bool_to_usize([
                                                          active_pair.contains(&eid[0]),
                                                          active_pair.contains(&eid[1]),
                                                          active_pair.contains(&eid[2]),
                                                          active_pair.contains(&eid[3]),
                                                          active_pair.contains(&eid[4]),
                                                      ], pattern_size);
            }

            matching.append(&mut path1_matching);
            matching.append(&mut path2_matching_forward);
            matching.append(&mut path2_matching_reverse);
            matching.append(&mut path2_matching_eq);

            if  pattern_size > 2  {
                matching.append(&mut path3_matching_forward);
                matching.append(&mut path1_path2_matching_forward);
                matching.append(&mut path3_matching_reverse);
                matching.append(&mut path2_path1_matching_forward);
                matching.append(&mut path3_matching_eq);


                if pattern_size >3 {

                    matching.append(&mut path4_matching_forward);
                    matching.append(&mut path4_matching_reverse);
                    matching.append(&mut path3_path1_matching_forward);
                    matching.append(&mut path2_path2_matching_forward);
                    matching.append(&mut path2_path2_matching_reverse);

                    // matching.append(&mut path2_path1_matching_forward);
                    matching.append(&mut path4_matching_eq);


                }
            }


            //matching.dedup_by_key(|m|m.eid);

            log(format!("Matching at time {:?},{:?}", &current_time, &matching), 4, DEBUG_FLAG);

            let  matching_count = matching.iter().filter(|m|m.match_size == pattern_size).count();
            matching = NFA::apply_partial_nfa(&nfa_join, &matching,dedup_flag);
            droped_match = matching_count - matching.iter().filter(|m|m.match_size == pattern_size).count();
            //let dic_state_2 = matching.iter().filter(|c| c.mid == 0).map(|m| m.state.clone()).collect_vec();
            dic_state.insert([0,0,0,0,0],matching.iter().filter(|c| c.mid == 0).map(|m| m.state.clone()).collect_vec());


            current_time = a.time;
            current_active = vec![];
            log(format!("{:?},{:?},{:?}", now.elapsed().as_secs_f32(),current_time, mid_counter), 1, DEBUG_FLAG);

        }
        if c <  actives.len() {
            current_active.push(a);
        }
    }
    let full_matching = matching.iter().filter(|m| m.match_size == pattern_size && m.eid[0] != 0).map(|m| (m.eid)).sorted_by(|m1,m2|m1.cmp(m2)).collect_vec();
    log(format!("Full Matching {:?}", &full_matching), 1, DEBUG_FLAG);
    log(format!("{:?},{:?}", now.elapsed().as_secs_f32(), full_matching.len()), 1,DEBUG_FLAG);

    println!("pt,n,{:?},{:?},{:?},{:?},{:?},{:?}/{:?}", &config.input_dir.replace("data/graphs/","").replace("/",""),pattern_type, nfa_filename.replace("data/nfa/","").replace(".csv",""),0,now.elapsed().as_secs_f32(), 0,full_matching.len());

    // let mut f = File::create("part.csv").unwrap();
    // for m in full_matching.into_iter().sorted_by(|m1,m2| m1.1.cmp(&m2.1)){
    //     f.write(format!(" {:?}\n", m.1).as_ref());
    //
    // }

}