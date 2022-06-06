mod error;

mod json_parser;
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

/// Tuple of `JSON` and `Binary buffer` (contains data used to play replay by WoT)
pub type ReplayParseResult<'a> = (Vec<&'a [u8]>, Vec<u8>);
