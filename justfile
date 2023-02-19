rp:
    cargo run --bin replay_parser_example

dev-rp:
    ../rustc_codegen_cranelift/dist/cargo-clif run --bin replay_parser_simple

rpr:
    cargo run --release --bin replay_parser_example

dp:
    cargo run --bin wot_datfile_parser_dev
fmt:
    cargo +nightly fmt

udeps:
    cargo +nightly udeps