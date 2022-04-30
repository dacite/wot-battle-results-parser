

pub fn main() {
    let file = std::fs::read("/home/dacite/Projects/wot-battle-results-parser/replay_parser/input_files/20220127_0851_germany-G58_VK4502P_10_hills.wotreplay").unwrap();

    let result = wot_replay_parser::parse(&file).unwrap();

    let packet_stream = wot_replay_parser::packet_stream::PacketStream::new(&result.1);

    for packet in packet_stream {
        println!("{:?}", packet);
    }
}
