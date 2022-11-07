mod error;
mod utils;

mod replay_errors;
mod replay_parser;

mod packet_parser;
pub use packet_parser::events;
pub use packet_parser::Event;
pub use packet_parser::EventStream;
pub use packet_parser::PacketStream;
pub use packet_parser::VERSIONS;

mod battle_context;
pub use battle_context::BattleContext;
pub use error::ReplayError;
/// TODO: Remove this * import
pub use replay_parser::*;

pub use crate::utils::get_replay_time;

pub mod wot_types {
    pub use wot_types::ArenaBonusType;
    pub use wot_types::WotValue;
}
