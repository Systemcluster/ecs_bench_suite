use bevy_ecs::{prelude::*, schedule::Schedule};

#[derive(Component)]
struct A(f32);
#[derive(Component)]
struct B(f32);
#[derive(Component)]
struct C(f32);
#[derive(Component)]
struct D(f32);
#[derive(Component)]
struct E(f32);

fn ab(mut query: Query<(&mut A, &mut B)>) {
    query.iter_mut().for_each(|(mut a, mut b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    })
}

fn cd(mut query: Query<(&mut C, &mut D)>) {
    query.iter_mut().for_each(|(mut c, mut d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    })
}

fn ce(mut query: Query<(&mut C, &mut E)>) {
    query.iter_mut().for_each(|(mut c, mut e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    })
}

pub struct Benchmark(World, Schedule);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.spawn_batch((0..crate::constants::SCHEDULE_ENTITIES).map(|_| (A(0.0), B(0.0))));
        world.spawn_batch((0..crate::constants::SCHEDULE_ENTITIES).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.spawn_batch((0..crate::constants::SCHEDULE_ENTITIES).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.spawn_batch((0..crate::constants::SCHEDULE_ENTITIES).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let mut schedule = Schedule::default();
        schedule.add_stage("main", SystemStage::parallel());
        schedule.add_system_to_stage("main", ab.system());
        schedule.add_system_to_stage("main", cd.system());
        schedule.add_system_to_stage("main", ce.system());

        Self(world, schedule)
    }

    pub fn run(&mut self) {
        self.1.run(&mut self.0);
    }
}
