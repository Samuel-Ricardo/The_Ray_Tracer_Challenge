use std::fs::write;

use crate::{
    canvas::{
        conversion::{png::Convertable, ppm::Convertible},
        model::{color::Color, Canvas},
    },
    scene::{
        model::{Environment, Pixel, Projectile},
        tick,
    },
    tuple::model::Tuple,
};

pub fn simulate_a_launch_and_plot_result_as_ppm() {
    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.02, 0.0, 0.0),
    );

    let projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    );

    let mut canvas = Canvas::new(900, 500);
    let color = Color::new(1.0, 1.0, 0.0);

    print!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;

    while current.position.y > 0.0 {
        match Pixel::from_point_for_canvas(current.position, &canvas) {
            Pixel::Coordinate { x, y } => canvas.write_pixel(x, y, color),
            Pixel::OutOfBounds { x, y } => {}
        }

        current = tick(&environment, &current);
        iteration += 1;
    }

    println!("[FINISHED] | SIMULATION => {}: {:?}", iteration, current);

    println!("[WRITING] => ./output.ppm");
    let ppm = canvas.to_ppm();
    write("./output.ppm", ppm).expect("Could not write ouput.ppm to disk.");

    let png = canvas.to_png();
    write("./output.png", png).expect("Could not write ouput.png to disk.");

    println!("[FINISHED] | FILE SAVED => [./output.ppm]");
}
