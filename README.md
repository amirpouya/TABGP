# TABGP
Protype for Sigmod Submition.

## How to run? 

cargo run --release --example baseline ./configs/simple_conf.toml

cargo run --release --example on-demand ./configs/simple_conf.toml

cargo run --release --example partial ./configs/simple_conf.toml

cargo run --release --example baseline-clock ./configs/simple_conf.toml

cargo run --release --example on-demand-clock ./configs/simple_conf.toml

cargo run --release --example partial-clock ./configs/simple_conf.toml


## Input Schema
Edge(edge.csv): eid: usize , src: usize, dst: usize, first: usize 
Active(active.csv): eid: usize, time: usize


## NFA Schema
NFA: state, [y1y2], next_state

TA: state, [y1y2], next_state, clock_set, clock_func,clock_cond

