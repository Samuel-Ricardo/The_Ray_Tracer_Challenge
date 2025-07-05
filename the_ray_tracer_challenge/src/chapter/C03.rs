use std::f64::consts::PI;
use std::fs::write;

use crate::{
    canvas::{
        self,
        conversion::{png::Convertable, ppm::Convertible},
        model::{color::Color, Canvas},
    },
    matrix::model::Matrix,
    scene::model::Pixel,
    tuple::model::Tuple,
};

const WIDTH: usize = 900;
const HEIGHT: usize = 500;

pub fn rendering_a_clock_cycle() {
    let mut canvas: Canvas = Canvas::new(WIDTH, HEIGHT);
    let color = Color::new(0.0, 1.0, 0.0);

    let new_origin = Tuple::point((WIDTH / 2) as f64, (HEIGHT / 2) as f64, 0.0);
    let origin_transform = Matrix::translation(new_origin.x, new_origin.y, new_origin.z);

    for hour in 0..12 {
        let root = 200.0;
        let rotation_transform = Matrix::rotation_z(2.0 * PI / 12.0 * (hour as f64));
        let point = Tuple::point(0.0, root, 0.0);

        let transformed_point = origin_transform * rotation_transform * point;

        //print!("Point: {:?}", transformed_point);

        match Pixel::from_point_for_canvas(transformed_point, &canvas) {
            Pixel::Coordinate { x, y } => canvas.write_pixel(x, y, color),
            Pixel::OutOfBounds { x, y } => panic!(
                "Could not map point to screen/canvas: Out of bounds: {:?} x {:?}",
                x, y
            ),
        }
    }

    println!("Writing ./clock.ppm");
    let ppm = canvas.to_ppm();
    write("./clock.ppm", ppm).expect("Could not write clock.ppm to disk.");

    println!("Writing ./clock.png");
    let png = canvas.to_png();
    write("./clock.png", png).expect("Could not write clock.png to disk.");
}
