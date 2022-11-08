use wot_replay_parser::ReplayParser;

pub fn main() {
    let path = "/home/dacite/Projects/wot-battle-results-parser/examples/example.wotreplay";

    // ReplayParser can take a path
    let replay_parser = ReplayParser::parse_file(path).unwrap();

    // Json returns serde_json::Value type
    let replay_json_start = replay_parser.replay_json_start().unwrap();
    let json_string = serde_json::to_string_pretty(&replay_json_start).unwrap();

    println!("{}", json_string);

    println!(
        "Replay Version: {:?}",
        replay_parser.parse_replay_version().unwrap()
    );
}
