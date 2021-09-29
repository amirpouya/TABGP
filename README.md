# TABGP
Prototype for Sigmod Submission.

## How to run? 

You need Rust to run this code
https://www.rust-lang.org/learn/get-started



cargo run --release --example baseline ./configs/simple_conf.toml

cargo run --release --example on-demand ./configs/simple_conf.toml

cargo run --release --example partial ./configs/simple_conf.toml

cargo run --release --example baseline-clock ./configs/simple_conf.toml

cargo run --release --example on-demand-clock ./configs/simple_conf.toml

cargo run --release --example partial-clock ./configs/simple_conf.toml


## Input Schema
- Edge(edge.csv): eid: usize , src: usize, dst: usize, first: usize 

- Active(active.csv): eid: usize, time: usize

Example: data/graphs/ex1 (represent the graph in Figure 1 in the paper).

## NFA Schema
NFA: state, [y1y2], next_state

TA: state, [y1y2], next_state, clock_set, clock_func,clock_cond

Example (data/nfa/ta2.csv, which represents the temporal automaton TA_2 introduced in the paper).

## Datasets

- EPL. https://github.com/jalapic/engsoccerdata
- EU-Email https://snap.stanford.edu/data/email-Eu-core-temporal.html
- FB-Wall http://socialnetworks.mpi-sws.org/data-wosn2009.html

