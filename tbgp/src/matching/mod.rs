use std::cmp::{max, min};
use std::collections::HashSet;

use array_tool::vec::Intersect;
use itertools::Itertools;

use crate::edge::Edge;
use crate::join::hash_join;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Matching {
    pub mid: usize,
    pub eid: [usize; 5],
    pub first: usize,
    pub last: usize,
    pub match_size: usize,
    pub state: usize,
    pub word: usize,
    pub clocks: [usize; 5],
}

#[derive(Debug)]
#[derive(Clone)]
pub struct NMatching {
    pub mid: usize,
    pub eid: [usize; 5],
    pub first: usize,
    pub last: usize,
    pub match_size: usize,
    pub state: HashSet<usize>,
    pub word: usize,
    pub clocks: [usize; 5],
}




impl Matching {
    pub fn new(mid: usize, eid: [usize; 5], first: usize, match_size: usize, last: usize) -> Self {
        Self {
            mid,
            eid: eid.clone(),
            first,
            match_size,
            last,
            state: 0,
            word: 0,
            clocks: [0, 0, 0, 0, 0],
        }
    }

    pub fn empty_match() -> Vec<Matching> {
        // Empty matching

        let m = Matching { mid: 0, eid: [0, 0, 0, 0, 0], first: 0, last: 0, match_size: 0, state: 0, word: 0, clocks: [0, 0, 0, 0, 0] };
        let matching: Vec<Matching> = vec![m];

        return matching;

    }

    pub fn MatchGen(edges: Vec<Edge>, pattern_type: &String) -> Vec<Matching> {
        let matching = match &pattern_type[..] {
            "instar_2" => Matching::in_star_two(edges.clone()),
            "outstar_2" => Matching::out_star_two(edges.clone()),
            "instar_3" => Matching::in_star_three(edges.clone()),
            "outstar_3" => Matching::out_star_three(edges.clone()),
            "triangles_WOJ" => Matching::triangles_woj(edges.clone()),
            "rectangle_woj" => Matching::rectangle_woj(edges.clone()),
            "path4" => Matching::path_four(edges.clone()),
            "path3" => Matching::path_three(edges.clone()),
            "path2" => Matching::path_two(edges.clone()),
            "cycle_two" | _ => Matching::cycle_two(edges.clone()),
        };
        matching
    }


    pub fn find_first_arr(input: [usize; 5]) -> usize
    {
        return input.iter().min().unwrap().clone();
    }
    pub fn find_last_arr(input: [usize; 5]) -> usize
    {
        return input.iter().max().unwrap().clone();
    }


    pub fn find_first(input: &Vec<&Edge>) -> usize
    {
        return input.iter().map(|e| e.first).min().unwrap();
    }
    pub fn find_last(input: &Vec<&Edge>) -> usize
    {
        return input.iter().map(|e| e.first).max().unwrap();
    }

    pub fn update_matching(matching: &Vec<Matching>, mids: &HashSet<usize>) -> Vec<Matching>
    {
        matching.iter()
            .filter(|m| mids.contains(&m.mid))
            .map(|m| Matching { mid: m.mid, eid: m.eid.clone(), first: m.first, last: m.last, match_size: m.match_size, state: m.state, word: 0, clocks: m.clocks })
            .collect_vec()
    }


    pub fn triangles_woj(edges: Vec<Edge>) -> Vec<Matching> {
        let src_index = Edge::get_src_index(&edges);
        let dst_index = Edge::get_dst_index(&edges);
        let src_dst = Edge::get_srt_dst_index(&edges);
        let mut matching: Vec<Matching> = vec![];

        let nodes = Edge::get_all_nodes(&edges);
        let mut mid: usize = 1;
        for x in nodes {
            let e = src_index.get(&x).unwrap_or(&Vec::new()).iter().map(|e| e.dst.clone()).collect_vec();
            for y in e {
                let z1 = dst_index.get(&x).unwrap_or(&Vec::new()).iter().map(|e| e.src.clone()).collect_vec();

                let z2 = src_index.get(&y).unwrap_or(&Vec::new()).iter().map(|e| e.dst.clone()).collect_vec();

                let zs = z1.intersect(z2);
                for z in zs {
                    let xy = src_dst.get(&(x, y)).unwrap();
                    let zx = src_dst.get(&(z, x)).unwrap();
                    let yz = src_dst.get(&(y, z)).unwrap();
                    let t = Matching::find_first(&vec![xy, zx, yz]);
                    let tf = Matching::find_last(&vec![xy, zx, yz]);
                    matching.push(Matching {
                        mid,
                        eid: [xy.eid, yz.eid, zx.eid, 0, 0],
                        first: t,
                        last: tf,
                        match_size: 3,
                        state: 0,
                        word: 0,
                        clocks: [0, 0, 0, 0, 0],
                    });
                    mid = mid + 1;
                }
            }
        }
        return matching;
    }


    pub fn rectangle_woj(edges: Vec<Edge>) -> Vec<Matching> {
        let src_index = Edge::get_src_index(&edges);
        let dst_index = Edge::get_dst_index(&edges);
        let src_dst = Edge::get_srt_dst_index(&edges);
        let mut matching: Vec<Matching> = vec![];

        let nodes = Edge::get_all_nodes(&edges);
        let mut mid: usize = 1;
        for w in nodes {
            let e = src_index.get(&w).unwrap_or(&Vec::new()).iter().map(|e| e.dst.clone()).collect_vec();
            for x in e {
                let e2 = src_index.get(&x).unwrap_or(&Vec::new()).iter().map(|e| e.dst.clone()).collect_vec();
                for y in e2 {
                    let z1 = dst_index.get(&w).unwrap_or(&Vec::new()).iter().map(|e| e.src.clone()).collect_vec();

                    let z2 = src_index.get(&y).unwrap_or(&Vec::new()).iter().map(|e| e.dst.clone()).collect_vec();

                    //let zs = join::intersect(&z1,&z2);
                    let zs = z1.intersect(z2);
                    for z in zs {
                        let wx = src_dst.get(&(w, x)).unwrap();
                        let xy = src_dst.get(&(x, y)).unwrap();
                        let zw = src_dst.get(&(z, w)).unwrap();
                        let yz = src_dst.get(&(y, z)).unwrap();
                        let t = Matching::find_first(&vec![wx, xy, yz, zw]);
                        let tf = Matching::find_last(&vec![wx, xy, yz, zw]);

                        matching.push(Matching {
                            mid,
                            eid: [wx.eid, xy.eid, yz.eid, zw.eid, 0, ],
                            //edge_hist:[wx.first, xy.first, wz.first, yz.first, 0, ],
                            first: t,
                            last: tf,
                            match_size: 4,
                            state: 0,
                            word: 0,
                            clocks: [0, 0, 0, 0, 0],
                        });
                        mid = mid + 1;
                    }
                }
            }
        }
        return matching;
    }


    pub fn cycle_two(edges: Vec<Edge>) -> Vec<Matching> {
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let matching = hash_join(&edges_dst.collect_vec(), &edges_src.collect_vec())
            .iter()
            .filter(|(e1, _, e2)| e1.src == e2.dst)
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }


    pub fn path_two(edges: Vec<Edge>) -> Vec<Matching> {
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let matching = hash_join(&edges_dst.collect_vec(), &edges_src.collect_vec())
            .iter()
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }


    pub fn in_star_two(edges: Vec<Edge>) -> Vec<Matching> {

        // R(z,x) S(z,y)
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone())).collect_vec();


        let matching = hash_join(&edges_src, &edges_src)
            .iter()
            .filter(|(x, _, y)| x.eid < y.eid)
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }

    pub fn in_star_three(edges: Vec<Edge>) -> Vec<Matching> {

        // R(w,x), R(w,y) S(x,z)
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone())).collect_vec();

        let half_matching = hash_join(&edges_src, &edges_src)
            .iter()
            .filter(|(x, _, y)| x.eid < y.eid)
            .map(|(x, w, y)| (*w, ([x.eid, y.eid], [x.first, y.first], usize::min(x.first, y.first), usize::max(x.first, y.first))))
            .collect_vec();

        let matching = hash_join(&half_matching, &edges_src)
            .iter()
            .enumerate()
            .filter(|(i, ((x, h, f, l), _, y))| x[0] < y.eid && x[1] < y.eid)
            .map(|(i, ((x, h, f, l), _, y))| Matching {
                mid: i + 1,
                eid: [x[0], x[1], y.eid, 0, 0],
                //edge_hist: [h[0], h[1], y.first, 0, 0],
                last: max(l.clone(), y.first)
                ,
                first: min(f.clone(), y.first),
                match_size: 3,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }

    pub fn out_star_two(edges: Vec<Edge>) -> Vec<Matching> {

        // R(y,x) S(z,x)


        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone())).collect_vec();

        let matching = hash_join(&edges_dst, &edges_dst)
            .iter()
            .filter(|(x, _, y)| x.eid < y.eid)
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }

    pub fn out_star_three(edges: Vec<Edge>) -> Vec<Matching> {

        // R(x,w), R(y,w) S(z,w)
        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone())).collect_vec();


        let half_matching = hash_join(&edges_dst, &edges_dst)
            .iter()
            .filter(|(x, _, y)| x.eid < y.eid)
            .map(|(x, w, y)| (*w, ([x.eid, y.eid], [x.first, y.first], usize::min(x.first, y.first), usize::max(x.first, y.first))))
            .collect_vec();

        let matching = hash_join(&half_matching, &edges_dst)
            .iter()
            .enumerate()
            .filter(|(i, ((x, h, f, l), _, y))| x[0] < y.eid && x[1] < y.eid)
            .map(|(i, ((x, h, f, l), _, y))| Matching {
                mid: i + 1,
                eid: [x[0], x[1], y.eid, 0, 0],
                //edge_hist: [h[0], h[1], y.first, 0, 0],
                last: max(l.clone(), y.first)
                ,
                first: min(f.clone(), y.first),
                match_size: 3,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }


    pub fn path_three(edges: Vec<Edge>) -> Vec<Matching> {
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let a = hash_join(&edges_dst.collect_vec(), &edges_src.clone().collect_vec())
            .into_iter()
            .enumerate()
            .map(|(i, (x, _, y))| (y.dst, ([x.eid, y.eid], [x.first, y.first], usize::min(x.first, y.first), usize::max(x.first, y.first))))
            .collect_vec();


        let matching = hash_join(&a, &edges_src.collect_vec())
            .iter()
            .enumerate()
            .map(|(i, ((x, h, f, l), _, y))| Matching {
                mid: i + 1,
                eid: [x[0], x[1], y.eid, 0, 0],
                //edge_hist: [h[0], h[1], y.first, 0, 0],
                last: max(l.clone(), y.first)
                ,
                first: min(f.clone(), y.first),
                match_size: 3,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
        //TODO: FIX HISTORTY
    }


    pub fn path_four(edges: Vec<Edge>) -> Vec<Matching> {
        let edges_src = edges.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let edges_dst = edges.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let a = hash_join(&edges_dst.collect_vec(), &edges_src.clone().collect_vec())
            .into_iter()
            .map(|(x, _, y)| (y.dst, ([x.eid, y.eid], [x.first, y.first], usize::min(x.first, y.first), usize::max(x.first, y.first))))
            .collect_vec();

        let b = hash_join(&a, &edges_src.clone().collect_vec())
            .into_iter()
            .map(|((x, h, f, l), _, y)| (y.dst, ([x[0], x[1], y.eid], [h[0], h[1], y.first], usize::min(f, y.first), usize::max(l, y.first))))
            .collect_vec();


        let matching = hash_join(&b, &edges_src.collect_vec())
            .iter()
            .enumerate()
            .map(|(i, ((x, h, f, l), _, y))| Matching {
                mid: i + 1,
                //edge_hist:[h[0],h[1],h[2],  y.first,0],
                eid: [x[0], x[1], x[2], y.eid, 0],
                first: min(f.clone(), y.first),
                last: max(l.clone(), y.first),
                match_size: 4,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        return matching;
    }


    pub fn path_two_dm(eid: usize, e: Vec<Edge>, de: Vec<Edge>) -> (usize, Vec<Matching>) {
        //TODO: Optimize for different cases
        let mut eid_max = eid;
        let e_src = e.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let e_dst = e.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let de_src = de.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let de_dst = de.iter()
            .map(|e| (e.dst.clone(), e.clone()));


        let mut e_de = hash_join(&e_dst.collect_vec(), &de_src.clone().collect_vec())
            .iter()
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: eid_max + i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + e_de.len();

        let mut de_e = hash_join(&de_dst.clone().collect_vec(), &e_src.collect_vec())
            .iter()
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: eid_max + i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + de_e.len();

        let mut de_de = hash_join(&de_dst.collect_vec(), &de_src.collect_vec())
            .iter()
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: eid_max + i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + de_de.len();


        e_de.append(&mut de_e);
        e_de.append(&mut de_de);
        return (eid_max, e_de);
    }


    pub fn cycle_two_dm(eid: usize, e: Vec<Edge>, de: Vec<Edge>) -> (usize, Vec<Matching>) {
        //TODO: Optimize for different cases
        let mut eid_max = eid;
        let e_src = e.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let e_dst = e.iter()
            .map(|e| (e.dst.clone(), e.clone()));

        let de_src = de.iter()
            .map(|e| (e.src.clone(), e.clone()));

        let de_dst = de.iter()
            .map(|e| (e.dst.clone(), e.clone()));


        let mut e_de = hash_join(&e_dst.collect_vec(), &de_src.clone().collect_vec())
            .iter()
            .filter(|(x, _, y)| x.src == y.dst)
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: eid_max + i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + e_de.len();

        let mut de_e = e_de
            .iter()
            .enumerate()
            .map(|(i, m)| Matching {
                mid: eid_max + i + 1,
                eid: [m.eid[1], m.eid[0], 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: m.first,
                last: m.last,
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + de_e.len();

        let mut de_de = hash_join(&de_dst.collect_vec(), &de_src.collect_vec())
            .iter()
            .filter(|(x, _, y)| x.src == y.dst)
            .enumerate()
            .map(|(i, (x, _, y))| Matching {
                mid: eid_max + i + 1,
                eid: [x.eid, y.eid, 0, 0, 0],
                //edge_hist: [x.first, y.first, 0, 0, 0],
                first: min(x.first, y.first),
                last: max(x.first, y.first),
                match_size: 2,
                state: 0,
                word: 0,
                clocks: [0, 0, 0, 0, 0],
            }).collect_vec();
        eid_max = eid_max + de_de.len();


        e_de.append(&mut de_e);
        e_de.append(&mut de_de);
        return (eid_max, e_de);
    }
}


#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct PMatching {
    pub mid: usize,
    pub eid: [usize; 5],
    pub first: usize,
    pub last: usize,
    pub match_size: usize,
    pub head: usize,
    pub head_idx: i8,
    pub tail: usize,
    pub tail_idx: i8,
    pub state: usize,
    pub word: usize,
    pub clocks: [usize; 5],
}

impl PMatching {
    pub fn fill_array(val: usize, idx: i8, inp: [usize; 5]) -> [usize; 5] {
        if idx == -1 {

            //println!("{:?}", inp)
        }
        let mut out = inp;
        out[idx as usize] = val;
        return out;
    }

    pub fn fill_array_with_vec(val: Vec<usize>, idx: usize, inp: [usize; 5]) -> [usize; 5] {
        let mut out = inp;
        val.iter().enumerate().for_each(|(i, e)| (
            {
                out[idx + i] = *e;
            }));
        return out;
    }
}