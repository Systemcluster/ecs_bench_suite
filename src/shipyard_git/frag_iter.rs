use shipyard_git::*;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            #[derive(Component)]
            struct $variants(f32);
            $world.bulk_add_entity((0..crate::constants::FRAG_ITER_ENTITIES).map(|_| ($variants(0.0), Data(1.0))));
        )*
    };
}

#[derive(Component)]
struct Data(f32);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        let (mut data,) = self.0.borrow::<(ViewMut<Data>,)>().unwrap();
        (&mut data).iter().for_each(|mut data| {
            data.0 *= 2.0;
        });
    }
}
