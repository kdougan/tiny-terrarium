#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct Fly;

#[derive(Debug, Clone, Copy)]
pub struct PathPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Path {
    pub points: Vec<PathPoint>,
    pub looped: bool,
}

#[derive(Debug)]
pub struct SporadicMovement {
    pub timer: f32,
    pub interval: f32,
}

#[derive(Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug)]
pub struct Appearance {
    pub color: raylib::color::Color,
    pub size: f32,
}

#[derive(Debug)]
pub struct Plant {
    pub type_: PlantType,
}

#[derive(Debug)]
pub enum PlantType {
    Tree,
    Flower,
    Weed,
    Grass,
}

#[derive(Debug)]
pub struct Growth {
    pub growth_rate: f32,
    pub max_size: f32,
}

#[derive(Debug)]
pub struct Weather {
    pub type_: WeatherType,
    pub duration: f32,
}

#[derive(Debug)]
pub enum WeatherType {
    Sunny,
    Rainy,
    Cloudy,
}
