use legion::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Transform([f32; 16]);

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Benchmark(World, Registry<u8>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.extend(
            (
                vec![Transform::default(); 1000],
                vec![Position::default(); 1000],
                vec![Rotation::default(); 1000],
                vec![Velocity::default(); 1000],
            )
                .into_soa(),
        );

        let mut registry = Registry::default();
        registry.register::<Transform>(0);
        registry.register::<Position>(1);
        registry.register::<Rotation>(2);
        registry.register::<Velocity>(3);

        Self(world, registry)
    }

    pub fn run(&mut self) {
        let Self(world, registry) = self;
        let canon = serialize::Canon::default();
        let serializable = &world.as_serializable(any(), &*registry, &canon);

        let serialized = ron::ser::to_string(serializable).unwrap();

        let mut deserializer = ron::de::Deserializer::from_str(&serialized).unwrap();
        registry
            .as_deserialize(&serialize::Canon::default())
            .deserialize(&mut deserializer)
            .unwrap();
    }
}
