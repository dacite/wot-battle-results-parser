use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub z: f32,
    pub y: f32,
}
