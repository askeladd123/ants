use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize)]
/// Information sent after wasm code was initialized
pub struct Init {
    pub buckets_meta: Vec<BucketMeta>,
}

#[derive(Clone, Default, Debug, Serialize)]
/// Information sent every game tick.
pub struct Tick {
    pub main: Main,
    // pub debug: Option<Debug>,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Main {
    pub buckets: Vec<Bucket>,
}

// #[derive(Clone, Default, Debug, Serialize)]
// pub struct Debug {
//     // pub buckets_meta: Vec<BucketMeta>,
// }

#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub a: f32,
    pub av: f32,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Bucket {
    pub index: u32,
    pub ants: Vec<Ant>,
}

#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct BucketMeta {
    pub index: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
