use crate::utils::{coord2index, index2coord, GridCentered};
use crate::web_output;
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

/// Data structure that stores buckets of sparse 2d objects in a uniform grid.
#[derive(Default)]
pub struct GridUniSparse<T: Indexed + Coord + Copy + Clone + Debug> {
    entries: Vec<HashMap<u32, T>>,
    index_counter: u32,
    grid: GridCentered,
}

/// Data structure that maps a coordinate system onto a uniform grid.
pub struct GridUniform {}

impl<T: Indexed + Coord + Copy + Clone + Debug> GridUniSparse<T> {
    /// * `rx` - distance from center to left/right edge
    /// * `ry` - distance from center to top/bottom edge
    pub fn new(grid: GridCentered) -> Self {
        info!(
            "creating grid\n\trx: {}\try: {}\n\tnx: {}\tny: {}\n\tvec: {}",
            grid.rx,
            grid.ry,
            grid.nx,
            grid.ny,
            grid.nx * grid.ny
        );
        // TODO: start from center
        Self {
            entries: vec![HashMap::new(); (grid.nx * grid.ny) as usize],
            index_counter: 0,
            grid,
        }
    }
    /// Create item in Gred, put into corresponding bucket / tile.
    pub fn new_item_at(&mut self, x: f32, y: f32) -> Result<()> {
        let item = T::new(self.index_counter);
        self.index_counter += 1;
        let index = coord2index(x, y, self.grid) as usize;
        self.entries[index].insert(item.index(), item);
        Ok(())
    }
    /// Scans all elements. Is positions updated, move into new bucket / tile.
    pub fn update(&mut self) -> Result<()> {
        let mut changed = Vec::new();
        for (i_old, bucket) in self.entries.iter().enumerate() {
            for (index, item) in bucket.iter() {
                let (x, y) = (item.x(), item.y());
                if x <= -self.grid.rx
                    || self.grid.rx <= x
                    || y <= -self.grid.ry
                    || self.grid.ry <= y
                {
                    continue;
                }
                let i_new = coord2index(item.x(), item.y(), self.grid);
                if i_new as usize != i_old {
                    changed.push((i_old, i_new, *index));
                }
            }
        }
        for (i_old, i_new, index) in changed.into_iter() {
            let item = self.entries[i_old as usize].remove(&index).unwrap();
            self.entries[i_new as usize].insert(item.index(), item);
        }
        Ok(())
    }
    pub fn items_from_bucket(&mut self, x: f32, y: f32) -> impl Iterator<Item = &mut T> {
        let index = coord2index(x, y, self.grid) as usize;
        self.entries[index].values_mut()
    }
    pub fn iter(&mut self) -> impl Iterator<Item = &T> {
        // TODO: is iter ambiguous, since structure is nested?
        // TODO: impl iter instead
        self.entries.iter().flat_map(|m| m.values())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        // TODO: impl iter instead
        self.entries.iter_mut().flat_map(|m| m.values_mut())
    }

    pub fn buckets(&self) -> impl Iterator<Item = &HashMap<u32, T>> {
        self.entries.iter()
    }
}

impl GridUniSparse<crate::Ant> {
    pub fn web_output_ants(&self) -> Vec<web_output::Ant> {
        let mut ants = Vec::with_capacity(self.index_counter as usize);
        for (i, bucket) in self.buckets().enumerate() {
            for ant in bucket.values() {
                ants.push(web_output::Ant {
                    x: ant.x,
                    y: ant.y,
                    v: ant.v,
                    a: ant.a,
                    av: ant.av,
                    has_food: false,
                    bucket_id: i as u32,
                });
            }
        }
        ants.shrink_to_fit();
        ants
    }
    pub fn web_output_buckets_debug_grid(&self) -> crate::web_output::Grid {
        let mut buckets = Vec::new();
        let (w, h) = (
            2. * self.grid.rx / self.grid.nx as f32,
            2. * self.grid.ry / self.grid.ny as f32,
        );
        for i in 0..self.entries.len() {
            let (x, y) = index2coord(i as u32, self.grid);
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
