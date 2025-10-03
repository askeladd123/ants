#![allow(dead_code)]
#![allow(unused)]

use rand::prelude::*;
use serde::Serialize;
use std::default;
use std::f64::consts::PI;
use std::panic;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize)]
pub struct Output {
    ants: Vec<Ant>,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Serialize)]
pub struct Ant {
    x: f32,
    y: f32,
    a: f32,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Environment {
    ants: Vec<Ant>,
    total_steps: i32,
}

#[wasm_bindgen]
impl Environment {
    #[wasm_bindgen(constructor)]
    pub fn new(ant_count: u32, width: i32, height: i32) -> Self {
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
            })
            .collect();
        Self {
            ants,
            ..Default::default()
        }
    }
    pub fn step(&mut self) -> JsValue {
        self.total_steps += 1;
        for ant in self.ants.iter_mut() {
            ant.x += (self.total_steps as f32 * 0.1).cos();
            ant.y += (self.total_steps as f32 * 0.1).sin();
            ant.a += (self.total_steps as f32 * 0.05).cos() * 0.1;
        }
        let out = Output {
            ants: self.ants.clone(), // TODO: can clone be avoided here?
        };
        serde_wasm_bindgen::to_value(&out).unwrap()
    }
}
