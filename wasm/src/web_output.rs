use serde::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct Output {
    pub main: Main,
    pub debug: Option<Debug>,
}

#[derive(Clone, Default, Serialize)]
pub struct Main {
    pub buckets: Vec<Bucket>,
}

#[derive(Clone, Default, Serialize)]
pub struct Debug {
    pub buckets_meta: Vec<BucketMeta>,
}

#[derive(Copy, Clone, Default, Serialize)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub a: f32,
    pub av: f32,
}

#[derive(Clone, Default, Serialize)]
pub struct Bucket {
    pub index: u32,
    pub ants: Vec<Ant>,
}

#[derive(Copy, Clone, Default, Serialize)]
pub struct BucketMeta {
    pub index: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
