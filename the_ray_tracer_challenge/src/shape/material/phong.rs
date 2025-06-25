use crate::canvas::model::color::Color;

use super::pattern::Patter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub pattern: Option<Patter>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shiness: f64,
    pub reflectiveness: f64,
}
