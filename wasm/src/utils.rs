use anyhow::Result;
use log::{debug, info};
use std::fmt::Debug;
use std::{collections::HashMap, ops::Index};

/// Can be positioned in a 2d coordinate system
pub trait Coord {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

/// Receive an index from and external source, and return it when asked
pub trait Indexed {
    fn new(index: u32) -> Self;
    fn index(&self) -> u32;
}

/// A uniform grid than gives you a subset of all elements based on buckets in a coordinate system.
#[derive(Default)]
pub struct Grid<T: Indexed + Coord + Copy + Clone + Debug> {
    entries: Vec<HashMap<u32, T>>,
    index_counter: u32,
    rx: f32,
    ry: f32,
    buckets_x: u32,
    buckets_y: u32,
}

impl<T: Indexed + Coord + Copy + Clone + Debug> Grid<T> {
    /// * `rx` - distance from center to left/right edge
    /// * `ry` - distance from center to top/bottom edge
    pub fn new(rx: f32, ry: f32, nx: u32, ny: u32) -> Self {
        info!(
            "creating grid\n\trx: {rx}\try: {ry}\n\tnx: {nx}\tny: {ny}\n\tvec: {}",
            nx * ny
        );
        // TODO: start from center
        Self {
            entries: vec![HashMap::new(); (nx * ny) as usize],
            index_counter: 0,
            rx,
            ry,
            buckets_x: nx,
            buckets_y: ny,
        }
    }
    /// Create item in Gred, put into corresponding bucket / tile.
    pub fn new_item_at(&mut self, x: f32, y: f32) -> Result<()> {
        let item = T::new(self.index_counter);
        self.index_counter += 1;
        let index = self.coord2index(x, y) as usize;
        self.entries[index].insert(item.index(), item);
        Ok(())
    }
    /// Scans all elements. Is positions updated, move into new bucket / tile.
    pub fn update(&mut self) -> Result<()> {
        let mut changed = Vec::new();
        for (i_old, bucket) in self.entries.iter().enumerate() {
            for (index, item) in bucket.iter() {
                let (x, y) = (item.x(), item.y());
                if x <= -self.rx || self.rx <= x || y <= -self.ry || self.ry <= y {
                    continue;
                }
                let i_new = self.coord2index(item.x(), item.y());
                if i_new as usize != i_old {
                    changed.push((i_old, i_new, *index));
                }
            }
        }
        for (i_old, i_new, index) in changed.into_iter() {
            let item = self.entries[i_old as usize].remove(&index).unwrap();
            // self.entries[i_new as usize].insert(item.index(), item);
            self.entries
                .get_mut(i_new as usize)
                .expect(&format!(
                    "index\n\told: {i_old}\n\tnew: {i_new}\nitem\n\tx: {}\n\ty: {}\ngrid\n\tn_x: {}\n\tn_y: {}\nmath\n\ti_x: {}\n\ti_y: {}",
                    item.x(), item.y(), self.buckets_x, self.buckets_y, (item.x() / (self.buckets_x - 1) as f32).floor(), (item.y() / (self.buckets_y - 1) as f32).floor()
                ))
                .insert(item.index(), item);
        }
        Ok(())
    }
    pub fn items_from_bucket(&mut self, x: f32, y: f32) -> impl Iterator<Item = &mut T> {
        let index = self.coord2index(x, y) as usize;
        self.entries[index].values_mut()
    }
    pub fn iter(&mut self) -> impl Iterator<Item = &T> {
        // TODO: impl iter instead
        self.entries.iter().flat_map(|m| m.values())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        // TODO: impl iter instead
        self.entries.iter_mut().flat_map(|m| m.values_mut())
    }
    /// for accessing row-major array
    fn coord2index(&self, x: f32, y: f32) -> u32 {
        // TODO: should `index` this be usize?
        // let i_x = ((x + self.rx) / (self.buckets_x - 1) as f32).floor() as u32;
        // let i_y = ((y + self.ry) / (self.buckets_y - 1) as f32).floor() as u32;
        let i_x = (self.buckets_x as f32 * ((x + self.rx) / (self.rx * 2.))) as u32;
        let i_y = (self.buckets_y as f32 * ((y + self.ry) / (self.ry * 2.))) as u32;
        i_x + i_y * self.buckets_x
    }
    /// for accessing row-major array
    fn index2coord(&self, i: u32) -> (f32, f32) {
        // TODO: should `index` this be usize?
        let i_x = i % self.buckets_x;
        let i_y = i / self.buckets_x;
        (
            (i_x as f32 * self.rx * 2.) / self.buckets_x as f32 - self.rx,
            (i_y as f32 * self.ry * 2.) / self.buckets_y as f32 - self.ry,
        )
    }
}

impl Grid<crate::Ant> {
    pub fn web_output_buckets(&self) -> Vec<crate::web_output::Bucket> {
        let mut buckets = Vec::new();

        for (i, map) in self.entries.iter().enumerate() {
            let mut bucket = crate::web_output::Bucket {
                index: i as u32,
                ants: Vec::new(),
            };
            for ant in map.values() {
                bucket.ants.push(crate::web_output::Ant {
                    x: ant.x,
                    y: ant.y,
                    a: ant.a,
                    av: ant.av,
                });
            }
            buckets.push(bucket);
        }
        buckets
    }
    pub fn web_output_buckets_debug_grid(&self) -> crate::web_output::Grid {
        let mut buckets = Vec::new();
        let (w, h) = (
            2. * self.rx / self.buckets_x as f32,
            2. * self.ry / self.buckets_y as f32,
        );
        for i in 0..self.entries.len() {
            let (x, y) = self.index2coord(i as u32);
            buckets.push(crate::web_output::BucketMeta {
                index: i as u32,
                x,
                y,
            });
        }
        crate::web_output::Grid {
            buckets,
            bucket_width: w,
            bucket_height: h,
        }
    }
}
