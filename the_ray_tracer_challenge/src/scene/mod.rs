use self::model::{Environment, Projectile};

pub mod model;

pub fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile::new(
        projectile.position + projectile.velocity,
        projectile.velocity + environment.gravity + environment.wind,
    )
}
