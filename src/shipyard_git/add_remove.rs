use shipyard_git::*;

#[derive(Component)]
struct A(f32);
#[derive(Component)]
struct B(f32);

pub struct Benchmark(World, Vec<EntityId>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entities = world
            .bulk_add_entity((0..crate::constants::ADD_REMOVE_ENTITIES).map(|_| (A(0.0),)))
            .collect();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        self.0
            .run(|entities: EntitiesViewMut, mut b: ViewMut<B>| {
                for entity in &self.1 {
                    entities.add_component(*entity, &mut b, B(0.0));
                }
            })
            .unwrap();

        self.0
            .run(|mut b: ViewMut<B>| {
                for entity in &self.1 {
                    b.remove(*entity);
                }
            })
            .unwrap();
    }
}
