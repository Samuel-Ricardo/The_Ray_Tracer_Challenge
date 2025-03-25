use crate::{
    scene::{
        model::{Environment, Projectile},
        tick,
    },
    tuple::model::Tuple,
};

pub fn simulate_physics_with_a_launch() {
    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.0001, 0.0, 0.0),
    );

    let projectile = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.02, 0.0, 0.0));

    println!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;

    // NOTE: Run simulation until we hit the ground
    while current.position.y > 0.0 {
        println!("{iteration}: {current:?}");
        current = tick(&environment, &current);
        iteration += 1;
        println!("FINISHED => {iteration}: {current:?}");
    }
}
