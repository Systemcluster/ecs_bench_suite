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

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.bulk_add_entity((0..crate::constants::SIMPLE_ITER_ENTITIES).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let (velocities, mut positions) = self.0.borrow::<(View<Velocity>, ViewMut<Position>)>().unwrap();
        (&velocities, &mut positions).iter().for_each(|(velocity, position)| {
            position.0 += velocity.0;
        });
    }
}
