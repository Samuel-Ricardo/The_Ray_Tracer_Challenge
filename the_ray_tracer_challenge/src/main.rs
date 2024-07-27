use chapter::{C01::simulate_physics_with_a_launch, C02::simulate_a_launch_and_plot_result_as_ppm};

mod canvas;
mod chapter;
mod scene;
mod tuple;
mod utils;

fn main() {
    // NOTE: [DEMO] | CHAPTER - 01 => simulate_physics_with_a_launch();

    simulate_a_launch_and_plot_result_as_ppm();
}
