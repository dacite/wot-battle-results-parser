rp:
    cargo run --bin replay_parser_example

rpr:
    cargo run --release --bin replay_parser_example

dp:
    cargo run --bin wot_datfile_parser_dev
fmt:
    cargo +nightly fmt

udep:
    cargo +nightly udeps