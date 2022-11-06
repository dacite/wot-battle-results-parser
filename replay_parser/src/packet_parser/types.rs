use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3 {
    x: f32,
    z: f32,
    y: f32,
}
