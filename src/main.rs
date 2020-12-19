use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod rect;
pub use rect::Rect;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();

        player_input(self, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        draw_map(&self.ecs, ctx);
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("title: S").build()?;

    rltk::main_loop(context, gs)
}
