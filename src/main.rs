use rltk::{GameState, Rltk, RGB};
use rltk_app::{
    components::{LeftMover, Player, Position, Renderable},
    map::{draw_map, new_map, TileType},
    player_handler::get_player_input,
    systems::LeftWalker,
};
use specs::{Builder, Join, RunNow, World, WorldExt};

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        get_player_input(&mut self.ecs, ctx, WIDTH, HEIGHT);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx, WIDTH);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn insert_resources(&mut self) {
        self.ecs.insert(new_map(WIDTH, HEIGHT));
    }

    fn register_components(&mut self) {
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<LeftMover>();
        self.ecs.register::<Player>();
    }

    fn generate_entities(&mut self) {
        // player
        self.ecs
            .create_entity()
            .with(Position {
                x: WIDTH / 2,
                y: HEIGHT / 2,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .build();

        // enemies
        for i in 0..10 {
            self.ecs
                .create_entity()
                .with(Position { x: i * 7, y: 20 })
                .with(Renderable {
                    glyph: rltk::to_cp437('☺'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                })
                .with(LeftMover {})
                .build();
        }
    }
}

fn build_context() -> rltk::Rltk {
    use rltk::RltkBuilder;
    RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .expect("Failed to build app context")
}

fn init_ecs() -> State {
    let mut gs = State { ecs: World::new() };
    gs.insert_resources();
    gs.register_components();
    gs.generate_entities();

    gs
}

fn main() -> rltk::BError {
    let context = build_context();
    let gs = init_ecs();

    rltk::main_loop(context, gs)
}
