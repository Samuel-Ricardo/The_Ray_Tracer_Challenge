use crate::{canvas::model::color::Color, utils::equality::FuzzyEq};

use super::pattern::Patter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub pattern: Option<Patter>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectiveness: f64,
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 0.0,
        }
    }
}
