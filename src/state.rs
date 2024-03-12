use raylib::prelude::Color;

pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub color: Color,
}

pub struct State {
    pub grid: Vec<Vec<Tile>>,
}

impl State {
    pub fn new(width: i32, height: i32) -> Self {
        let mut grid = Vec::new();
        for x in 0..width {
            let mut row = Vec::new();
            for y in 0..height {
                row.push(Tile {
                    x,
                    y,
                    size: 16,
                    color: Color::new(100, 100, 100, 255),
                });
            }
            grid.push(row);
        }
        Self { grid }
    }
}
