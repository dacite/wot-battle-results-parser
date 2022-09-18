mod error;
mod json_parser;
mod utils;
pub use json_parser::{JsonParser, ReplaySummary};

pub mod packet_parser;
mod replay_parser;
pub use packet_parser::events;
pub use packet_parser::events::EventStream;
pub use packet_parser::PacketStream;

mod battle_context;
pub mod def_parser;
pub use battle_context::{get_replay_time, BattleContext};
pub use error::Error;
pub use error::Result;
pub use replay_parser::*;

pub static VERSIONS: &[[u16; 4]] = &[[0, 9, 15, 0], [1, 6, 1, 0], [1, 8, 0, 0], [1, 16, 1, 0]];
