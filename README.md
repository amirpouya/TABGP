# TABGP
Protype for Sigmod Submition.


cargo run --release --example basline ./configs/simple_conf.toml

cargo run --release --example on-demand ./configs/simple_conf.toml

cargo run --release --example partial ./configs/simple_conf.toml

cargo run --release --example basline-clock ./configs/simple_conf.toml

cargo run --release --example on-demand-clock ./configs/simple_conf.toml

cargo run --release --example partial-clock ./configs/simple_conf.toml


## Graph Schema
Edge : eid: usize , src: usize, dst: usize, first: usize 

Active: eid: usize, time: usize


