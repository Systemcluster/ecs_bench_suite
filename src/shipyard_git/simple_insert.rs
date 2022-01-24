use cgmath::*;
use shipyard_git::*;

#[derive(Clone, Copy, Component)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy, Component)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::default();

        world.bulk_add_entity((0..crate::constants::SIMPLE_INSERT_ENTITIES).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));
    }
}
