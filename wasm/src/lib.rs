#![allow(dead_code)]
#![allow(unused)]

use rand::prelude::*;
use serde::Serialize;
use std::default;
use std::panic;
use wasm_bindgen::prelude::*;

const PI: f32 = std::f64::consts::PI as f32;

#[derive(Clone, Serialize)]
pub struct Output {
    ants: Vec<Ant>,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Default, Serialize)]
pub struct Ant {
    // TODO: separate into Internal and External ant (because JS don't need all fields)
    x: f32,
    y: f32,
    a: f32,  // angle: radians
    av: f32, // angular velocity: radians per tick
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Environment {
    ants: Vec<Ant>,
    total_steps: i32,
    ant_speed: f32,
    rng: ThreadRng,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl Environment {
    #[wasm_bindgen(constructor)]
    pub fn new(ant_count: u32, width: i32, height: i32) -> Self {
        // TODO: make parameter one object
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let right = width as f32 / 2.;
        let left = -width as f32 / 2.;
        let up = height as f32 / 2.;
        let down = -height as f32 / 2.;
        let mut rng = rand::rng();
        let ants = (0..ant_count)
            .map(|_| Ant {
                x: rng.random_range(left..right),
                y: rng.random_range(down..up),
                a: rng.random_range(0f32..(PI as f32 * 2.)),
                ..Default::default()
            })
            .collect();
        Self {
            ants,
            ant_speed: 0.1,
            rng: rng,
            width: width as f32,
            height: height as f32,
            ..Default::default()
        }
    }
    pub fn step(&mut self) -> JsValue {
        self.total_steps += 1;
        for ant in self.ants.iter_mut() {
            if self.rng.random_ratio(1, 80) {
                const MARGIN: f32 = PI / 50.;
                ant.av = self.rng.random_range(-MARGIN..MARGIN);
            }
            if self.rng.random_ratio(1, 40) {
                ant.av = 0.;
            }
            ant.a += ant.av;
            ant.x += ant.a.cos();
            ant.y += ant.a.sin();

            boundary_collision(
                &mut ant.x,
                &mut ant.y,
                &mut ant.a,
                -self.width / 2.,
                self.width / 2.,
                -self.height / 2.,
                self.height / 2.,
            );
        }
        let out = Output {
            ants: self.ants.clone(), // TODO: can clone be avoided here?
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
