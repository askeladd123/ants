use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize)]
/// Information sent every game tick.
pub struct Output {
    pub main: Main,
    pub debug: Option<Debug>,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Main {
    pub buckets: Vec<Bucket>,
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Debug {
    pub grid: Grid,
}

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
