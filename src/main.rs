use rltk::{GameState, Rltk, RGB};
use rltk_app::components::{Position, Renderable};
use specs::prelude::*;
use specs::World;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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

fn generate_entities(gs: &mut State) {
    // player
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    // enemies
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
}

fn register_components(gs: &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
}

fn init_ecs() -> State {
    let mut gs = State { ecs: World::new() };
    register_components(&mut gs);
    generate_entities(&mut gs);

    gs
}

fn main() -> rltk::BError {
    let context = build_context();
    let gs = init_ecs();

    rltk::main_loop(context, gs)
}
