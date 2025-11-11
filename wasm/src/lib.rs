#![allow(dead_code)]
#![allow(unused)]

use log::{debug, info, trace};
use rand::prelude::*;
use serde::Serialize;
use std::default;
use std::fmt::Display;
use std::panic;
use wasm_bindgen::prelude::*;

use crate::data_structures::{Coord, GridUniSparse, GridUniform, Indexed};
use crate::utils::{GridCentered, Tile};

mod data_structures;
mod utils;
mod web_output;

const PI: f32 = std::f64::consts::PI as f32;
/// how much angle can randomly change
const ANT_AV_MARGIN: f32 = PI;
const ANT_V_MAX: f32 = 100.0;
const ANT_V_MIN: f32 = 60.0;

/// * `x` - position: left right
/// * `y` - position: up down
/// * `v` - velocity: around 1.0
/// * `a` - angle radians
/// * `av` - angular velocity, radians per tick
#[derive(Copy, Clone, Debug)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub v: f32,
    pub a: f32,
    pub av: f32,
    pub index: u32,
}

impl Coord for Ant {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}

impl Indexed for Ant {
    fn new(index: u32) -> Self {
        let mut rng = rand::rng();

        Self {
            index,
            x: 0.,
            y: 0.,
            v: rng.random_range(ANT_V_MIN..ANT_V_MAX),
            a: rng.random_range(0f32..PI * 2.),
            av: 0.,
        }
    }

    fn index(&self) -> u32 {
        self.index
    }
}

#[wasm_bindgen]
pub struct Environment {
    grid_uni_sparse: GridUniSparse<Ant>,
    grid_uniform: GridUniform<Tile>,
    total_steps: i64,
    rng: ThreadRng,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl Environment {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ant_count: u32,
        width: i32,
        height: i32,
        grid_uni_sparse_nx: u32,
        grid_uni_sparse_ny: u32,
        grid_uniform_nx: u32,
        grid_uniform_ny: u32,
    ) -> Self {
        console_log::init_with_level(log::Level::Trace);
        // TODO: make parameter one object
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        info!("creating environment\n\tants: {ant_count}\n\tw: {width}\th: {height}");

        let mut rng = rand::rng();

        let grid_centered = GridCentered {
            rx: width as f32 / 2.,
            ry: height as f32 / 2.,
            nx: grid_uni_sparse_nx,
            ny: grid_uni_sparse_ny,
        };

        let mut grid_uni_sparse = GridUniSparse::<Ant>::new(grid_centered);
        for i in 0..ant_count {
            grid_uni_sparse.new_item_at(0., 0.);
        }

        let grid_centered = GridCentered {
            rx: grid_centered.rx,
            ry: grid_centered.ry,
            nx: grid_uni_sparse_nx,
            ny: grid_uni_sparse_ny,
        };
        let mut grid_uniform = GridUniform::new(grid_centered);
        for y in ((grid_centered.ry * 0.7) as u32)..((grid_centered.ry * 0.8) as u32) {
            for x in ((grid_centered.rx * 0.8) as u32)..((grid_centered.rx * 0.9) as u32) {
                grid_uniform.set(x as f32, y as f32, Tile::Food { amount: 100 });
            }
        }

        Self {
            grid_uni_sparse,
            grid_uniform,
            total_steps: 0,
            rng: rng,
            width: width as f32,
            height: height as f32,
        }
    }

    /// * `delta` - time between frame: for consistant speed accross different framerates
    pub fn step(&mut self, delta: f32, debug_mode: bool) -> JsValue {
        self.total_steps += 1;

        if self.total_steps % 60 == 0 {
            self.grid_uni_sparse.update();
        }

        for mut ant in self.grid_uni_sparse.iter_mut() {
            if self.rng.random_ratio(1, 80) {
                ant.av = self.rng.random_range(-ANT_AV_MARGIN..ANT_AV_MARGIN);
                ant.v = self.rng.random_range(ANT_V_MIN..ANT_V_MAX);
            }
            if self.rng.random_ratio(1, 40) {
                ant.av = 0.;
            }
            ant.a += ant.av * delta;
            ant.x += ant.a.cos() * ant.v * delta;
            ant.y += ant.a.sin() * ant.v * delta;

            let l = -self.width / 2.;
            let r = self.width / 2.;
            let u = -self.height / 2.;
            let d = self.height / 2.;

            boundary_collision(&mut ant.x, &mut ant.y, &mut ant.a, l, r, u, d);
        }
        let out = web_output::Output {
            main: web_output::Main {
                buckets: self.grid_uni_sparse.web_output_buckets(),
            },
            debug: if debug_mode {
                Some(web_output::Debug {
                    grid_uni_sparse: self.grid_uni_sparse.web_output_buckets_debug_grid(),
                    grid_uniform: self.grid_uniform.clone(),
                })
            } else {
                None
            },
        };
        serde_wasm_bindgen::to_value(&out).unwrap()
    }
}

fn boundary_collision(x: &mut f32, y: &mut f32, a: &mut f32, l: f32, r: f32, u: f32, d: f32) {
    const MARGIN: f32 = 1.;
    // left
    let diff = l - *x;
    if 0. < diff {
        *x += diff + MARGIN;
        *a = PI - *a;
    }
    // right
    let diff = r - *x;
    if diff < 0. {
        *x -= diff + MARGIN;
        *a = PI - *a;
    }
    // up
    let diff = u - *y;
    if 0. < diff {
        *y += diff + MARGIN;
        *a = -*a;
    }
    // down
    let diff = d - *y;
    if diff < 0. {
        *y -= diff + MARGIN;
        *a = -*a;
    }
}
