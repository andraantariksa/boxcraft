use rapier3d::prelude::*;

pub struct Config {
    // Physics
    pub physics_pipeline_active: bool,
    pub query_pipeline_active: bool,
    pub gravity: Vector<Real>,
}

impl Config {
    fn new() -> Self {
        Self {
            physics_pipeline_active: true,
            query_pipeline_active: true,
            gravity: Vector::new(0.0, -9.81, 0.0),
        }
    }
}
