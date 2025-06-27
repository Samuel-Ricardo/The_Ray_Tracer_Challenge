use chapter::{C02::simulate_a_launch_and_plot_result_as_ppm, C03::rendering_a_clock_cycle};

#[macro_use]

mod canvas;
mod chapter;
mod matrix;
mod ray_tracer;
mod scene;
mod shape;
mod tuple;
mod utils;

fn main() {
    // NOTE: [DEMO] | CHAPTER - 01 => simulate_physics_with_a_launch();

    // NOTE: [DEMO] | CHAPTER - 02 => simulate_a_launch_and_plot_result_as_ppm();
    simulate_a_launch_and_plot_result_as_ppm();

    // NOTE: [DEMO] | CHAPTER - 03 => renderingAClockCycle();
    rendering_a_clock_cycle();
}
