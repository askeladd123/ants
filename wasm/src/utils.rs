use serde::Serialize;

/// * `nx` - amount of tiles in the horizontal direction
/// * `ny` - amount of tiles in the vertical direction
/// * `rx` - world distance from center to left/right edge
/// * `ry` - world distance from center to top/bottom edge
#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct GridCentered {
    pub nx: u32,
    pub ny: u32,
    pub rx: f32,
    pub ry: f32,
}

/// for accessing row-major array spread over a 2d coordinate system
pub fn coord2index(x: f32, y: f32, grid: GridCentered) -> u32 {
    // TODO: return type should be `usize`
    let i_x = (grid.nx as f32 * ((x + grid.rx) / (grid.rx * 2.))) as u32;
    let i_y = (grid.ny as f32 * ((y + grid.ry) / (grid.ry * 2.))) as u32;
    i_x + i_y * grid.nx
}

/// for accessing row-major array spread over a 2d coordinate system
pub fn index2coord(i: u32, grid: GridCentered) -> (f32, f32) {
    // TODO: should `index` this be usize?
    let i_x = i % grid.nx;
    let i_y = i / grid.nx;
    (
        (i_x as f32 * grid.rx * 2.) / grid.nx as f32 - grid.rx,
        (i_y as f32 * grid.ry * 2.) / grid.ny as f32 - grid.ry,
    )
}

#[derive(Default, Copy, Clone, Debug, Serialize)]
pub enum Tile {
    #[default]
    Empty,
    Pheromone {
        r#type: Pheromone,
        strength: f32,
        dir_x: f32,
        dir_y: f32,
    },
    Food {
        amount: u32,
    },
    Wall,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum Pheromone {
    ToFood,
    ToHome,
}
