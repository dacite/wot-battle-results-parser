mod error;
mod utils;

pub mod packet_parser;
mod replay_parser;
pub use packet_parser::events;
pub use packet_parser::events::EventStream;
pub use packet_parser::PacketStream;

mod battle_context;
pub use battle_context::BattleContext;
pub use error::Error;
pub use error::Result;
pub use replay_parser::*;

pub use crate::utils::get_replay_time;
pub use crate::events::method_defs::METHOD_MAP;