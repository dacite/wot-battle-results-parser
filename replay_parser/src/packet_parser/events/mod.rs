/// `entity_method` describe multiple events because there can be many different types of method calls
mod entity_method;
pub use entity_method::vehicle_methods::*;
pub use entity_method::EntityMethod;
pub use entity_method::EntityMethodEvent;
pub use entity_method::avatar_methods::update_arena::ArenaUpdateData;

mod game_version;
pub use game_version::GameVersion;

mod avatar_create;
pub use avatar_create::AvatarCreate;

mod position;
pub use position::Position;

mod chat;
pub use chat::Chat;

mod entity_create;
pub use entity_create::EntityCreate;

mod crypto_key;
pub use crypto_key::CryptoKey;
