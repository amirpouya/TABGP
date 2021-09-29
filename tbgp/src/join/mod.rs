use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;

// If you know one of the tables is smaller, it is best to make it the second parameter.
pub fn hash_join<A, B, K>(first: &[(K, A)], second: &[(K, B)]) -> Vec<(A, K, B)>
    where
        K: Hash + Eq + Copy,
        A: Copy,
        B: Copy,
{
    let mut hash_map = HashMap::new();

    // hash phase
    for &(key, val_a) in second {
        // collect all values by their keys, appending new ones to each existing entry
        hash_map.entry(key).or_insert_with(Vec::new).push(val_a);
    }

    let mut result = Vec::new();
    // join phase
    for &(key, val_b) in first {
        if let Some(vals) = hash_map.get(&key) {
            let tuples = vals.iter().map(|&val_a| (val_b, key, val_a));
            result.extend(tuples);
        }
    }
    result
}



fn anti_join<A, B, K>(first: &[(K, A)], second: &[(K, B)]) -> Vec<K>
    where
        K: Hash + Eq + Copy,
        A: Copy,
        B: Copy,
{
    let mut hash_map = HashMap::new();

    // hash phase
    for &(key, val_a) in second {
        // collect all values by their keys, appending new ones to each existing entry
        hash_map.entry(key).or_insert_with(Vec::new).push(val_a);
    }

    let mut result = Vec::new();
    // join phase
    for &(key, _) in first {
        if !hash_map.contains_key(&key){
            result.push(key);
        }
    }

    result
}

pub fn intersect(a:&Vec<usize>, b:&Vec<usize>) -> Vec<usize>
{
    let a: HashSet<usize> =a.clone().into_iter().collect();
    let b: HashSet<usize> = b.clone().into_iter().collect();

    let intersection = a.intersection(&b);
    intersection.map(|e| *e).collect_vec()
}



