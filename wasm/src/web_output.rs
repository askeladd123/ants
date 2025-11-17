use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

/// * `x` - position: left right
/// * `y` - position: up down
/// * `v` - velocity: around 1.0
/// * `a` - angle radians
/// * `av` - angular velocity, radians per tick
/// * `has_food` - is ant carrying food home, or out looking
/// * `bucket_id` - location in underlying data structure
#[wasm_bindgen]
#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub v: f32,
    pub a: f32,
    pub av: f32,
    pub has_food: bool,
    pub bucket_id: u32,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Grid {
    pub buckets: Vec<BucketMeta>,
    pub bucket_width: f32,
    pub bucket_height: f32,
}

#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct BucketMeta {
    pub index: u32,
    pub x: f32,
    pub y: f32,
}
