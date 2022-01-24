use cgmath::*;
use rayon::prelude::*;
use shipyard_git::*;

#[derive(Clone, Copy, Component)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Velocity(Vector3<f32>);

#[derive(Clone, Copy, Component)]
struct Mat(Matrix4<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.bulk_add_entity((0..crate::constants::HEAVY_COMPUTE_ENTITIES).map(|_| {
            (
                Mat(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let (mut positions, mut transforms) = self.0.borrow::<(ViewMut<Position>, ViewMut<Mat>)>().unwrap();
        (&mut positions, &mut transforms)
            .par_iter()
            .for_each(|(mut pos, mut mat)| {
                for _ in 0..100 {
                    mat.0 = mat.0.invert().unwrap();
                }
                pos.0 = mat.0.transform_vector(pos.0);
            });
    }
}
