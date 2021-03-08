use dear_gui::AppInit;
use glium::Surface;

pub mod agent;
pub mod entity;
pub mod grid;
pub mod tile;
pub mod world;
pub mod resources;
pub mod building;

use grid::CanvasGrid;
use world::World;

fn main() {
    let mut app = AppInit::new();

    app.set_canvas_click_handler({
        // let state = state.clone();
        Box::new(move |pos| {
            println!("User clicked: {:?}", pos);
        })
    });

    // let mut ui = UI::new(app.imgui.clone());

    let mut grid = CanvasGrid::new(&app.display, 10, 10);
    let mut world = World::new(320, 320, 10_000);
    let mut i = 0;

    for i in 0..100_000 {
        if i % 10000 == 0 {
            dbg!(i);
        }
        world.step();
    }

    app.run(move |app, target, _last_frame| {
        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();

        if i == 0 {
            world.step();
        }
        i = (i + 1) % 10;
        world.update_grid(&app.display, &mut grid);

        // ui.draw(last_frame, target, &state, &mut cmd);
    });
}
